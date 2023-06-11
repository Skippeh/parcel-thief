#![allow(non_snake_case, clippy::too_many_arguments)] // extern functions do be

use std::ptr;

snek::snek! {
    Oodle {
        OodleLZ_Compress: (
            compressor: Compressor,
            src: *const u8,
            src_len: i64,
            dest: *mut u8,
            dest_len: i64,
            level: CompressionLevel,
            options: *const usize,
            offsets: *const usize,
            unused: *const usize,
            scratch: *const usize,
            scratch_size: i64
        ) -> i64,
        OodleLZ_Decompress: (
            src: *const u8,
            src_len: i64,
            dest: *mut u8,
            dest_len: i64,
            fuzz: i32,
            crc: i32,
            verbose: i32,
            context: *const usize,
            e: usize,
            callback: *const usize,
            callback_ctx: *const usize,
            scratch: *const usize,
            scratch_size: i64,
            thread_phase: ThreadPhase
        ) -> i64,
        OodleLZ_GetCompressedBufferSizeNeeded: (src_len: i32) -> i32,
        OodleLZDecoder_MemorySizeNeeded: (compressor: Compressor, len: i32) -> i32
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
#[allow(dead_code)]
pub enum ThreadPhase {
    Invalid,
    ThreadPhase1,
    ThreadPhase2,
    Unthreaded,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
#[allow(dead_code)]
pub enum CompressionLevel {
    None,
    SuperFast,
    VeryFast,
    Fast,
    Normal,
    Optimal1,
    Optimal2,
    Optimal3,
    Optimal4,
    Optimal5,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
#[allow(dead_code)]
pub enum Compressor {
    Invalid = -1,
    Lzh = 0,
    Lzhlw = 1,
    Lznib = 2,
    None = 3,
    LZB16 = 4,
    Lzblw = 5,
    Lza = 6,
    Lzna = 7,
    Kraken = 8,
    Mermaid = 9,
    BitKnit = 10,
    Selkie = 11,
    Hydra = 12,
    Leviathan = 13,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
#[allow(dead_code)]
pub enum Verbosity {
    None,
    Minimal,
    Some,
    Lots,
}

#[derive(Debug, thiserror::Error)]
pub enum OodleError {
    #[error("Compression failed")]
    CompressionFailed,
    #[error("Decompression failed. Check DebugView or similar programs to get more info.")]
    DecompressionFailed,
    #[error("Could not call dll: {0:?}")]
    DllError(snek::Error),
}

impl From<snek::Error> for OodleError {
    fn from(value: snek::Error) -> Self {
        Self::DllError(value)
    }
}

#[allow(dead_code)]
pub fn compress(
    compressor: Option<Compressor>,
    level: Option<CompressionLevel>,
    src: &[u8],
) -> Result<Vec<u8>, OodleError> {
    let mut dest = vec![0u8; get_needed_compressed_buffer_size(src.len() as i32)? as usize];
    let dest_len = compress_to(compressor, level, src, &mut dest)?;
    dest.truncate(dest_len);
    dest.shrink_to_fit();

    Ok(dest)
}

#[allow(dead_code)]
pub fn compress_to(
    compressor: Option<Compressor>,
    level: Option<CompressionLevel>,
    src: &[u8],
    dest: &mut [u8],
) -> Result<usize, OodleError> {
    let compressor = compressor.unwrap_or(Compressor::Kraken);
    let src_len = src.len() as i64;
    let src = std::pin::pin!(src).as_ptr();
    let dest_len = dest.len() as i64;
    let dest = std::pin::pin!(dest).as_mut_ptr();
    let level = level.unwrap_or(CompressionLevel::Optimal2);

    unsafe {
        let oodle = load_oodle()?;
        let dest_size = oodle.OodleLZ_Compress(
            compressor,
            src,
            src_len,
            dest,
            dest_len,
            level,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            0,
        );

        if dest_size == 0 {
            Err(OodleError::CompressionFailed)
        } else {
            Ok(dest_size as usize)
        }
    }
}

fn load_oodle() -> Result<Oodle<'static>, snek::Error> {
    Oodle::load("oo2core_7_win64.dll")
}

pub fn decompress(
    src: &[u8],
    dest_len: usize,
    fuzz_safe: bool,
    check_crc: bool,
) -> Result<Vec<u8>, OodleError> {
    let mut dest = vec![0u8; dest_len];
    decompress_to(src, &mut dest, fuzz_safe, check_crc)?;

    Ok(dest)
}

pub fn decompress_to(
    src: &[u8],
    dest: &mut [u8],
    fuzz_safe: bool,
    check_crc: bool,
) -> Result<usize, OodleError> {
    let src_len = src.len() as i64;
    let src = std::pin::pin!(src).as_ptr();
    let dest_len = dest.len() as i64;
    let dest = std::pin::pin!(dest).as_mut_ptr();
    let fuzz = if fuzz_safe { 1 } else { 0 };
    let crc = if check_crc { 1 } else { 0 };
    let verbosity = Verbosity::Lots as i32;

    unsafe {
        let oodle = load_oodle()?;
        let dest_size = oodle.OodleLZ_Decompress(
            src,
            src_len,
            dest,
            dest_len,
            fuzz,
            crc,
            verbosity,
            ptr::null(),
            0,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            0,
            ThreadPhase::Unthreaded,
        );

        if dest_size == 0 {
            Err(OodleError::DecompressionFailed)
        } else {
            Ok(dest_size as usize)
        }
    }
}

pub fn get_needed_compressed_buffer_size(src_len: i32) -> Result<i32, OodleError> {
    let oodle = load_oodle()?;
    unsafe { Ok(oodle.OodleLZ_GetCompressedBufferSizeNeeded(src_len)) }
}
