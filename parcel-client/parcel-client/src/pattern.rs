use std::fmt::Display;

pub fn find_single(
    start_abs_addr: usize,
    search_length: usize,
    pattern: &str,
) -> Result<Option<usize>, MemoryReaderError> {
    let reader = MemoryReader::new(start_abs_addr, search_length)?;
    patternscan::scan_first_match(reader, pattern)
        .map_err(|err| err.into())
        .map(|addr| addr.map(|addr| addr + start_abs_addr))
}

pub fn find_many(
    start_abs_addr: usize,
    search_length: usize,
    pattern: &str,
) -> Result<Vec<usize>, MemoryReaderError> {
    let reader = MemoryReader::new(start_abs_addr, search_length)?;
    patternscan::scan(reader, pattern)
        .map_err(|err| err.into())
        .map(|res| res.into_iter().map(|val| val + start_abs_addr).collect())
}

struct MemoryReader {
    start_addr: usize,
    len: usize,
    cur_pos: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryReaderError {
    RangeOutOfBounds,
    PatternScanError(patternscan::Error),
}

impl Display for MemoryReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryReaderError::RangeOutOfBounds => write!(
                f,
                "The address range is not within the bounds of the module"
            ),
            MemoryReaderError::PatternScanError(err) => write!(f, "{}", err),
        }
    }
}

impl From<patternscan::Error> for MemoryReaderError {
    fn from(value: patternscan::Error) -> Self {
        MemoryReaderError::PatternScanError(value)
    }
}

impl MemoryReader {
    pub fn new(start_addr: usize, len: usize) -> Result<Self, MemoryReaderError> {
        Ok(Self {
            start_addr,
            len,
            cur_pos: start_addr,
        })
    }
}

impl std::io::Read for MemoryReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let max_read = (self.start_addr + self.len) - self.cur_pos;

        if max_read == 0 {
            return Ok(0);
        }

        let read_len = buf.len().clamp(0, max_read);

        unsafe {
            let slice = std::slice::from_raw_parts(self.cur_pos as *const u8, read_len);
            buf[0..read_len].copy_from_slice(slice);
            self.cur_pos += read_len;
        }

        Ok(read_len)
    }
}
