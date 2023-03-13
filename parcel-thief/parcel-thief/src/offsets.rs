use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    sync::{Arc, RwLock},
};

use anyhow::Context;
use lazy_static::lazy_static;
use windows::{
    w,
    Win32::System::{
        Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER},
        LibraryLoader::GetModuleHandleW,
        SystemServices::IMAGE_DOS_HEADER,
    },
};

use crate::pattern::MemoryReaderError;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, enum_display_derive::Display)]
pub enum LocationOffset {
    FnStringCtor,
    FnStringDtor,

    FnReadIncomingData,
    FnWriteOutgoingData,

    DataAuthUrlPtr,
}

pub fn map_offsets() -> Result<(), anyhow::Error> {
    let offsets = &mut OFFSETS.write().unwrap();

    offsets
        .map_pattern_offset(
            LocationOffset::FnStringCtor,
            "40 53 48 83 EC 20 48 8B D9 48 C7 01 00 00 00 00 49 C7 C0 FF FF FF FF",
        )
        .context("Failed to find String::ctor offset")?;

    offsets
        .map_pattern_offset(
            LocationOffset::FnStringDtor,
            "40 53 48 83 EC 20 48 8B 19 48 8D 05 ? ? ? ? 48 83 EB 10",
        )
        .context("Failed to find String::dtor offset")?;

    offsets
        .map_pattern_offset(
            LocationOffset::FnReadIncomingData,
            "48 89 5C 24 08 55 56 57 41 56 41 57 48 83 EC 50 48 8B 05 D9 50 58 03 48 33 C4 48 89",
        )
        .context("Failed to find read_incoming_data offset")?;
    offsets
        .map_pattern_offset(
            LocationOffset::FnWriteOutgoingData,
            "48 89 5C 24 08 48 89 74 24 20 55 57 41 56 48 8B EC 48 81 EC 80 00 00 00 48 8B 05 11",
        )
        .context("Failed to find write_outgoing_data offset")?;

    // Map DataAuthUrlPtr
    let (start, end) = offsets
        .get_data_section()
        .expect(".data section should always exist");
    let auth_url_offset = crate::pattern::find_single(
        start,
        end - start,
        // https://prod-pc-15.wws-gs2.com/auth/ds\0
        "68 74 74 70 73 3a 2f 2f 70 72 6f 64 2d 70 63 2d 31 35 2e \
        77 77 73 2d 67 73 32 2e 63 6f 6d 2f 61 75 74 68 2f 64 73 00",
    );

    match auth_url_offset {
        Ok(Some(mut addr)) => {
            addr = offsets
                .get_relative_addr(addr)
                .expect("Should always return a valid address");

            addr += 39 + 1; // length of auth url including terminator and +1 padding
            offsets.map_offset(LocationOffset::DataAuthUrlPtr, addr)?;
        }
        _ => anyhow::bail!("Could not find auth url pointer offset"),
    }

    Ok(())
}

lazy_static! {
    pub static ref OFFSETS: Arc<RwLock<Offsets<LocationOffset>>> =
        Arc::new(RwLock::new(Offsets::new()));
}

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
    MemoryReaderError(MemoryReaderError),
    PatternNotFound,
}

impl From<MemoryReaderError> for OffsetsError {
    fn from(value: MemoryReaderError) -> Self {
        Self::MemoryReaderError(value)
    }
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
            OffsetsError::MemoryReaderError(err) => write!(f, "Pattern error: {}", err),
            OffsetsError::PatternNotFound => write!(f, "The pattern could not be found"),
        }
    }
}

#[derive(Debug)]
pub struct Offsets<MapKey>
where
    MapKey: Copy + Eq + PartialEq + Hash + Display,
{
    module_base: usize,
    module_size: usize,
    section_ranges: HashMap<String, (usize, usize)>,
    mapped_offsets: HashMap<MapKey, usize>,
}

impl<MapKey> Offsets<MapKey>
where
    MapKey: Copy + Eq + PartialEq + Hash + Display,
{
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
    pub fn map_offset(&mut self, name: MapKey, relative_addr: usize) -> Result<(), OffsetsError> {
        if self.mapped_offsets.contains_key(&name) {
            return Err(OffsetsError::KeyAlreadyMapped);
        }

        let absolute_addr = self.get_absolute_addr(relative_addr)?;
        self.mapped_offsets.insert(name, absolute_addr);
        println!(
            "Mapped {} to 0x{:X?} (0x{:X?})",
            name, absolute_addr, relative_addr
        );

        Ok(())
    }

    pub fn map_pattern_offset(&mut self, name: MapKey, pattern: &str) -> Result<(), OffsetsError> {
        let (start, end) = self.get_module_range();
        let len = end - start;

        if self.mapped_offsets.contains_key(&name) {
            return Err(OffsetsError::KeyAlreadyMapped);
        }

        let offset = crate::pattern::find_single(start, len, pattern)?;

        match offset {
            Some(offset) => {
                let relative_offset = offset - start;
                self.map_offset(name, relative_offset)?;
                Ok(())
            }
            None => Err(OffsetsError::PatternNotFound),
        }
    }

    pub fn get_mapped_offset(&self, name: MapKey) -> usize {
        self.mapped_offsets
            .get(&name)
            .copied()
            .expect("Expected known MapKey variant")
    }

    pub unsafe fn cast_mapped_offset<T>(&self, name: MapKey) -> &'static T {
        self.mapped_offsets
            .get(&name)
            .map(|addr| &*(addr as *const usize).cast::<T>())
            .expect("Expected known MapKey variant")
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
