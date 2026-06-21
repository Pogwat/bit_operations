//self[index] == *(self[index]) not (*self)[index] , deref on index return not self
use crate::BitOps;

pub struct MutBitProxy<'a,ElementType:BitOps> {
    val:bool,
    addr: &'a mut ElementType,
    bit:u8    
}

use std::ops::Deref;

impl<'a,ElementType:BitOps> Deref for MutBitProxy<'a,ElementType> {
    type Target = bool;
    fn deref(&self) -> &Self::Target {&self.val} //Cant mutate cuz &self
}

use std::ops::DerefMut;

impl <'a,ElementType:BitOps> DerefMut for MutBitProxy<'a,ElementType> {
    fn deref_mut(&mut self) -> &mut Self::Target {&mut self.val}
}

impl<'a, ElementType:BitOps> Drop for MutBitProxy<'a, ElementType> {
    fn drop(&mut self) {self.addr.set_bit(self.bit as usize, self.val)}
}

impl <'a,ElementType:BitOps> MutBitProxy<'a,ElementType> {
    pub fn new(addr:&'a mut ElementType,bit:usize) -> Self {
        Self {val: addr.get_bit(bit),addr,bit:bit as u8}
    }
}