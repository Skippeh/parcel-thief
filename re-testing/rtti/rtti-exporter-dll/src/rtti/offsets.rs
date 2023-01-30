use std::{collections::HashMap, fmt::Display};

use windows::{
    w,
    Win32::System::{
        Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER},
        LibraryLoader::GetModuleHandleW,
        SystemServices::IMAGE_DOS_HEADER,
    },
};

/// Get field offset in struct
macro_rules! get_offset {
    ($type:ty, $field:tt) => {{
        let dummy = ::core::mem::MaybeUninit::<$type>::uninit();

        let dummy_ptr = dummy.as_ptr();
        let member_ptr = ::core::ptr::addr_of!((*dummy_ptr).$field);

        member_ptr as usize - dummy_ptr as usize
    }};
}

#[derive(Debug, thiserror::Error)]
pub enum OffsetsError {
    AddressOutOfBounds(usize),
    KeyAlreadyMapped,
}

impl Display for OffsetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffsetsError::AddressOutOfBounds(addr) => write!(
                f,
                "The address {:X?} is not within the bounds of the module",
                addr
            ),
            OffsetsError::KeyAlreadyMapped => {
                write!(f, "The given key is already mapped to an address")
            }
        }
    }
}

#[derive(Debug)]
pub struct Offsets {
    module_base: usize,
    module_size: usize,
    section_ranges: HashMap<String, (usize, usize)>,
    mapped_offsets: HashMap<String, usize>,
}

impl Offsets {
    pub fn new() -> Self {
        unsafe {
            let module = GetModuleHandleW(w!("ds.exe")).unwrap();
            let dos_header = &*(module.0 as *const IMAGE_DOS_HEADER);
            let nt_headers =
                &*((module.0 + (dos_header.e_lfanew as isize)) as *const IMAGE_NT_HEADERS64);

            Self {
                module_base: module.0 as usize,
                module_size: nt_headers.OptionalHeader.SizeOfImage as usize,
                section_ranges: Self::get_sections(nt_headers, module.0 as usize),
                mapped_offsets: HashMap::new(),
            }
        }
    }

    pub fn get_sections(
        headers: &IMAGE_NT_HEADERS64,
        module_base: usize,
    ) -> HashMap<String, (usize, usize)> {
        unsafe {
            let mut result = HashMap::new();
            let mut first_sector_addr = headers as *const IMAGE_NT_HEADERS64 as usize;
            first_sector_addr += get_offset!(IMAGE_NT_HEADERS64, OptionalHeader);
            first_sector_addr += headers.FileHeader.SizeOfOptionalHeader as usize;
            let mut section_addr = first_sector_addr as *const IMAGE_SECTION_HEADER;

            for _ in 0..headers.FileHeader.NumberOfSections {
                let section = &*(section_addr);
                let str_len = memchr::memchr(b'\0', &section.Name).unwrap_or(section.Name.len());
                let name = String::from_utf8_lossy(&section.Name[0..str_len]);

                result.insert(
                    name.into_owned(),
                    (
                        module_base + section.VirtualAddress as usize,
                        (module_base
                            + section.VirtualAddress as usize
                            + section.Misc.VirtualSize as usize),
                    ),
                );

                section_addr = section_addr.add(1);
            }

            result
        }
    }

    /// Translates a module base relative address to an absolute one
    pub fn get_absolute_addr(&self, addr: usize) -> Result<usize, OffsetsError> {
        let (start, end) = self.get_module_range();

        if start + addr >= end {
            Err(OffsetsError::AddressOutOfBounds(start + addr))
        } else {
            Ok(start + addr)
        }
    }

    /// Translates an absolute address to an address relative to the module's base address
    pub fn get_relative_addr(&self, addr: usize) -> Result<usize, OffsetsError> {
        let (start, end) = self.get_module_range();

        if addr < start || addr >= start + end {
            Err(OffsetsError::AddressOutOfBounds(addr))
        } else {
            Ok(addr - start)
        }
    }

    #[inline]
    pub fn get_code_section(&self) -> Option<(usize, usize)> {
        self.get_section_range(".text")
    }

    #[inline]
    pub fn get_data_section(&self) -> Option<(usize, usize)> {
        self.get_section_range(".data")
    }

    #[inline]
    pub fn get_readonly_data_section(&self) -> Option<(usize, usize)> {
        self.get_section_range(".rdata")
    }

    /// Returns true if the given address is within the module
    #[inline]
    pub fn addr_in_module(&self, addr: usize) -> bool {
        addr >= self.module_base && addr < self.module_base + self.module_size
    }

    /// Returns true if the given range is witin the module. Note that the upper bound check is inclusive.
    #[inline]
    pub fn range_in_module(&self, addr: usize, len: usize) -> bool {
        addr >= self.module_base && addr + len <= self.module_base + self.module_size
    }

    /// Returns the module's start and end address
    #[inline]
    pub fn get_module_range(&self) -> (usize, usize) {
        (self.module_base, self.module_base + self.module_size)
    }

    /// Returns the section's start and end address
    #[inline]
    pub fn get_section_range(&self, section_name: &str) -> Option<(usize, usize)> {
        self.section_ranges.get(section_name).copied()
    }

    /// Maps a relative offset with a given name. When retrieved later with get_mapped_offset the absolute address will be returned.
    pub fn map_offset(&mut self, name: &str, relative_addr: usize) -> Result<(), OffsetsError> {
        if self.mapped_offsets.contains_key(name) {
            return Err(OffsetsError::KeyAlreadyMapped);
        }

        let absolute_addr = self.get_absolute_addr(relative_addr)?;
        self.mapped_offsets.insert(name.into(), absolute_addr);

        Ok(())
    }

    pub fn get_mapped_offset(&self, name: &str) -> Option<usize> {
        self.mapped_offsets.get(name).copied()
    }

    /// Gets the name of the section the given address resides in, if any.
    pub fn get_address_section(&self, addr: usize) -> Option<&str> {
        for (name, (start, end)) in &self.section_ranges {
            if &addr >= start && &addr < end {
                return Some(name);
            }
        }

        None
    }
}
