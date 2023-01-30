mod offsets;
pub mod pattern;
mod rtti_types;

use std::{
    fmt,
    sync::{Arc, RwLock},
};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use num::FromPrimitive;
use num_derive::FromPrimitive;

use crate::{
    libc_util::read_cstring,
    rtti::rtti_types::{class::RttiClass, container::RttiContainer, primitive::RttiPrimitive},
};

use self::offsets::Offsets;

#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum InfoType {
    Primitive = 0,
    Reference = 1,
    Container = 2,
    Enum = 3,
    Class = 4,
    EnumFlags = 5,
    /// Plain old data
    Pod = 6,
    Max,
}

#[repr(C)]
pub union EnumTypeSizeOrClassInheritanceCount {
    pub enum_underlying_type_size: u8,
    pub class_inheritance_count: u8,
}

impl fmt::Debug for EnumTypeSizeOrClassInheritanceCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "enum_underlying_type_size/class_inheritance_count: {}",
            unsafe { self.enum_underlying_type_size } // both union fields are the same size so this is safe
        )
    }
}

#[repr(C)]
pub union CountsUnion {
    pub enum_member_count: u16,
    pub class_count: ClassCount,
}

impl fmt::Debug for CountsUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            // both union fields are the same size so this is safe
            write!(
                f,
                "enum_member_count: {}, class_count: {:?}",
                self.enum_member_count, self.class_count
            )
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ClassCount {
    pub class_member_count: u8,
    pub lua_function_count: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct RttiBase {
    pub _runtime_type_id_1: u16, // 0xFFFF
    pub _runtime_type_id_2: u16, // 0xFFFF
    pub info_type_raw: u8,
    pub enum_type_size_or_class_inheritance_count: EnumTypeSizeOrClassInheritanceCount,
    pub counts_union: CountsUnion,
}

impl RttiBase {
    #[inline]
    pub fn get_info_type(&self) -> Option<InfoType> {
        <InfoType as FromPrimitive>::from_u8(self.info_type_raw)
    }
}

pub enum RttiType {
    Container(&'static RttiContainer),
    Class(&'static RttiClass),
}

lazy_static! {
    pub static ref OFFSETS: Arc<RwLock<Offsets>> = Arc::new(RwLock::new(Offsets::new()));
}

pub unsafe fn export() -> Result<()> {
    let (data_start, data_end) = OFFSETS
        .read()
        .unwrap()
        .get_data_section()
        .context("could not find .data section")?;
    let (rdata_start, rdata_end) = OFFSETS
        .read()
        .unwrap()
        .get_readonly_data_section()
        .context("could not find .rdata section")?;

    let mut rtti_types = Vec::new();
    let rtti_matches =
        pattern::find_many(data_start, data_end - data_start, "FF FF FF FF ? ? ? ?")?;

    let is_data_segment = |addr: usize| -> bool { addr >= data_start && addr < data_end };
    let is_rdata_segment = |addr: usize| -> bool { addr >= rdata_start && addr < rdata_end };

    println!("potential rtti matches: {}", rtti_matches.len());

    for addr in rtti_matches {
        let rtti = &*(addr as *const RttiBase);

        if rtti.info_type_raw < (InfoType::Primitive as u8)
            || rtti.info_type_raw >= (InfoType::Max as u8)
        {
            continue;
        }

        match rtti.get_info_type() {
            None => continue,
            Some(info_type) => match info_type {
                InfoType::Container | InfoType::Reference => {
                    let container = &*(addr as *const RttiContainer);

                    if !is_data_segment(container.ty as usize)
                        || !is_data_segment(container.data as usize)
                    {
                        continue;
                    }

                    rtti_types.push(RttiType::Container(container));
                }
                InfoType::Class => {
                    let class = &*(addr as *const RttiClass);

                    if (class.name as usize != 0 && !is_rdata_segment(class.name as usize))
                        || class.alignment == 0
                    {
                        continue;
                    }

                    let mut member_ptr = class.member_table;

                    for _ in 0..class.base.counts_union.class_count.class_member_count {
                        let member = &*member_ptr;

                        if !is_rdata_segment(member.name as usize) {
                            continue;
                        }

                        let member_name = read_cstring(member.name);
                        println!("{:X?}", member_name);

                        member_ptr = member_ptr.add(1);
                    }

                    rtti_types.push(RttiType::Class(class));
                }
                InfoType::Primitive => {
                    let primitive = &*(addr as *const RttiPrimitive);

                    if !is_rdata_segment(primitive.name as usize) {
                        continue;
                    }

                    let name_str = read_cstring(primitive.name);

                    //println!("{:?}: {:#?}", name_str, primitive);
                }
                InfoType::Max => unreachable!(),
                _ => continue,
            },
        }

        //println!("0x{:X?}: {:#?}", addr, rtti);
    }

    Ok(())
}
