//self[index] == *(self[index]) not (*self)[index] , deref on index return not self
use crate::BitOps;
//Proxy For Bit Mutating
pub struct MutBitProxy<'a,ElementType:BitOps> {
    ///Bit being derefed to
    val:bool,
    ///Address to write back to on Drop
    addr: &'a mut ElementType,
    ///Bit in *Address to write val to
    bit:u8
}

use core::ops::{Deref,DerefMut};

impl<'a,ElementType:BitOps> Deref for MutBitProxy<'a,ElementType> {
    type Target = bool;
    fn deref(&self) -> &Self::Target {&self.val} //Cant mutate cuz &self
}

impl <'a,ElementType:BitOps> DerefMut for MutBitProxy<'a,ElementType> {
    fn deref_mut(&mut self) -> &mut Self::Target {&mut self.val}
}

impl<'a, ElementType:BitOps> Drop for MutBitProxy<'a, ElementType> {
    fn drop(&mut self) {self.addr.set_bit(self.bit, self.val)}
}

impl <'a,ElementType:BitOps> MutBitProxy<'a,ElementType> {
    ///New proxy
    pub fn new(addr:&'a mut ElementType,bit:u8) -> Self {
        Self {val: addr.get_bit(bit),addr,bit}
    }
}
