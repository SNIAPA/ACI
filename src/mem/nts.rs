use std::fmt;

#[repr(packed)]
#[derive(Debug)]
pub struct Nts([u8;255]);


//TODO: shit error handling
impl fmt::Display for Nts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp = self.0[0..self.0.iter().copied().position(|x| {x == 0}).unwrap() as usize].to_vec();
        write!(f,"{}",String::from_utf8(tmp).unwrap())
        
    }
    
}
