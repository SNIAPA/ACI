use std::{
    mem::transmute, fs::File,os::unix::prelude::FileExt, io::{Read, Seek, SeekFrom}, ptr
};

use chrono::Utc;

use crate::{util::Result, log};


pub trait BetterReading {
    fn read_pointer(&self, address: u64) -> Result<u32>;
    fn read_string(&mut self, offsets: Vec<u64>) -> Result<String>;
}

impl BetterReading for File {
    fn read_pointer(&self, address: u64) -> Result<u32>{
        let mut buf = [0u8; 4];
        self.read_at(&mut buf, address)?;

        Ok(unsafe { transmute::<[u8; 4], u32>(buf)})
    }

    fn read_string(&mut self, mut offsets: Vec<u64>) -> Result<String> {
        if offsets.len() == 1 {
            let mut buf = [0u8; 255];

            self.seek(SeekFrom::Start(offsets[0]))?;
            self.read_exact(&mut buf)?;

            let null_terminator_index = buf.iter().position(|&x| {x == 0}).ok_or("invalid stirng or too long, no null terminator")?;
            let buf = &buf[..null_terminator_index];

            return Ok(std::str::from_utf8(buf)?.to_owned());
        }

        let value = self.read_pointer(offsets[0])?;
         
        offsets[1] += value as u64;
        self.read_string(offsets[1..].to_vec())
    }
}

pub fn derefrence_pointer<T>(address: usize, offsets: impl IntoIterator<Item = usize>) -> *mut T{

    let mut ptr = address as *mut usize;
    for offset in offsets {
        log!("{:X}", (*ptr) as usize);
        unsafe{
          ptr = (*ptr + offset) as *mut usize;
        }
        log!("{:X}", ptr as usize);
    }

    unsafe { ptr as *mut T}
}
