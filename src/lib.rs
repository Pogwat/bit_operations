#![no_std]
#[doc = include_str!("../README.md")]
pub mod mut_proxy;
pub use intops::IntOps;
pub use mut_proxy::MutBitProxy;
pub trait BitOps:IntOps { //Todo: Make this trait const when #143874 is staballized: https://github.com/rust-lang/rust/issues/143874
    /// number of bits required to address a bit in this type
    const BIT_BITS:u32 = Self::BITS.ilog2();
    /// Generate a bitmask for aa range of bits
    fn bitmask<R:BitZRange<Self> >(range:&R) -> Self { //indexes: 0..=Self::BITS-1
        let start = range.bits_start();
        let end = range.bits_end();
        (!Self::ZERO >> (Self::BITS as u8 - 1 - (end - start))) << start
    }
    /// Get a specifc bit by bit index (0 indexed)
    fn get_bit(&self, bitdex:u8) -> bool {(*self & Self::ONE<<bitdex) !=Self::ZERO }
    /// Set a specifc bit by bit index (0 indexed)
    fn set_bit(&mut self, bitdex:u8, val:bool) {
        let mask = Self::ONE<<bitdex;
        *self = (*self & !mask) | Self::from(val)<<bitdex; //Clear bit then set it
    }
    /// Get bits in range (0 indexed)
    fn get_bits<R:BitZRange<Self> >(&self, range:&R) -> Self {Self::bitmask(range) & *self}
    /// count 0 bits in range (0 indexed)
    fn ctz<R:BitZRange<Self> >(&self, range:&R) -> u8 {
        ((!Self::bitmask(range)) | *self).count_zeros() as u8 //  111111BitsWeWant1111111 , others are 1
    }
    /// count 1 bits in range (0 indexed)
    fn popcnt<R:BitZRange<Self> >(&self, range:&R) -> u8 {
        self.get_bits(range).count_ones() as u8
    }
    /// set bits in range to val (0 indexed)
    fn set_bits<R:BitZRange<Self> >(&mut self, range:&R, val:bool) {
        self.set_these_bits(Self::set_all_bit(val),range)
    }
    /// set all bits to val
    fn set_all_bit(val:bool) -> Self {(Self::ZERO).wrapping_sub(Self::from(val))}
    /// set a specfic range of self to these bits (0 indexed)
    fn set_these_bits<R:BitZRange<Self> >(&mut self, bits:Self, range:&R) {
        //XOR is commutative and self-inverse
        //A ^B ^B  = A ^(B^B), B^B = 0, So A^B^B = A , dobule xoring undos xor
        //Here we Self^Bits and truncate it, then we xor it to reverse the xors, giving us self and truncated bits
        let diff = (*self ^ bits) & Self::bitmask(range); //Truncated diff
        *self ^= diff; //XORing the diff undo the xor leaving just a truncated bits and self
    }
    /// get the first set bit can go OOB
    fn first_set_bit(&self) -> u8 {self.trailing_zeros() as u8} //Can go OOB
    /// get the last set bit can go OOB
    fn last_set_bit(&self) -> u8 {(Self::BITS -1 - self.leading_zeros()) as u8} //Can go OOB
    /// get mutable ref to type using proxy, MUST DROP REF FOR BIT TO UPDATE!!!!
    fn mut_bit(&mut self, bit:u8) -> MutBitProxy<'_,Self> {MutBitProxy::<Self>::new(self,bit)} //Returns proxy struct, on drop proxy updates bit
    /// first one in a bit range
    fn first_one<R:BitZRange<Self>>(&self, range:&R) -> Option<u8> {
        let masked =  *self & Self::bitmask(range); //0000BitsWeWant0000
       (masked!=Self::ZERO).then_some(masked.trailing_zeros() as u8)
    }
    /// first zero in a bit range
    fn first_zero<R:BitZRange<Self>>(&self, range:&R) -> Option<u8> {
        let masked =  *self | !Self::bitmask(range); //1111BitsWeWant1111
       (masked!=!Self::ZERO).then_some(masked.trailing_ones() as u8)
    }


}
impl <T:IntOps>BitOps for T {}

use core::ops::{Bound,RangeBounds};

pub trait NumRangeExtract<T:IntOps>: RangeBounds<T>  {
    fn end(&self) -> Option<T> {
        match self.end_bound() {
            Bound::Included(val) => Some(*val),
            Bound::Excluded(val) => Some((*val).saturating_sub(T::ONE)),
            Bound::Unbounded =>  None
        }
    }
    fn start(&self) -> Option<T> {
        match self.start_bound() {
            Bound::Included(val) => Some(*val),
            Bound::Excluded(val) => Some((*val).saturating_add(T::ONE)), // Rare in standard Rust ranges, but possible
            Bound::Unbounded => None           // e.g., ..5 starts at index 0
        }
    }
}
impl <T:IntOps, R:RangeBounds<T>>NumRangeExtract<T> for R {}

pub trait BitZRange<ElementType:BitOps> :NumRangeExtract<u8> {
    fn bits_start(&self) -> u8 {self.start().unwrap_or(0).max(0)}
    fn bits_end(&self) -> u8 {self.end().unwrap_or(ElementType::BITS as u8-1).min(ElementType::BITS as u8-1)}
}
impl <ElementType:BitOps, R:NumRangeExtract<u8> > BitZRange<ElementType> for R {}
