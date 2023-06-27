use std::ffi::{CString, CStr};

use crate::{util::{Result, error::CheatError}, log};

pub type nts = [u8;255];

pub fn new_nts(val: &str) -> nts{
    let mut res = [0u8;255];
    res[..val.len()].copy_from_slice(&val.as_bytes());
    res
}

pub trait to_string {
    fn as_string(&self) -> Result<String>;
}

impl to_string for nts {
    fn as_string(&self) -> Result<String> {
        let tmp = self[0..self.iter().copied().position(|x| {x == 0}).ok_or(CheatError::new("invalid nts".to_owned()))? as usize].to_vec();
        unsafe{
            Ok(String::from_utf8(tmp)?)
        }
    }
}
