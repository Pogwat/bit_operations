#![no_std]
#[doc = include_str!("../README.md")]
pub mod mut_proxy;
pub use mut_proxy::MutBitProxy;
pub trait BitOps:BitTypes {
    /// number of bits the type has
    const TYPE_BITS:usize;
    /// number of bits required to address a bit in this type
    const BIT_BITS:usize;
    /// Generate a bitmask for aa range of bits
    fn bitmask<R:RangeBounds<usize>+ NumRangeExtract<usize>>(range: &R) -> Self;
    /// Get a specifc bit by bit index (0 indexed)
    fn get_bit(&self, bitdex:usize) -> bool;
    /// Set a specifc bit by bit index (0 indexed)
    fn set_bit(&mut self, bitdex:usize, val:bool);
    /// Get bits in range (0 indexed)
    fn get_bits<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> Self;
    /// count 0 bits in range (0 indexed)
    fn ctz<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> usize;
    /// count 1 bits in range (0 indexed)
    fn popcnt<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> usize;
    /// set bits in range to val (0 indexed)
    fn set_bits<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&mut self, range:&R, val:bool);
    /// set all bits to val
    fn set_all_bit(val:bool) -> Self;
    /// set a specfic range of self to these bits (0 indexed)
    fn set_these_bits<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&mut self, bits:Self, range:&R);
    /// get the first set bit can go OOB
    fn first_set_bit(&self) -> usize;
    /// get the last set bit can go OOB
    fn last_set_bit(&self) -> usize;
    /// get mutable ref to type using proxy, MUST DROP REF FOR BIT TO UPDATE!!!!
    fn mut_bit(&mut self, bit:usize) -> MutBitProxy<'_,Self>;
}
use core::ops::{Shl,Sub,BitXor,Not};
pub trait BitTypes: Sized+Shl<usize, Output = Self> + Sub<Self, Output = Self> + BitXor<Self, Output = Self> +  Not{}

macro_rules! bittypes {
    ($($type:ty),*) => {
        $(
            impl BitTypes for $type {}

            impl BitOps for $type {
                const TYPE_BITS:usize = Self::BITS as usize;
                const BIT_BITS:usize = Self::BITS.ilog2() as usize;

                fn bitmask<R:RangeBounds<usize>+ NumRangeExtract<usize>>(range:&R) -> Self { //indexes: 0..=Self::BITS-1
                    let start = range.bits_start();
                    let end = range.bits_end();
                    (Self::MAX >> (Self::BITS as usize - 1 - (end - start))) << start
                }

                fn get_bit(&self, bitdex:usize) -> bool {(self & 1<<bitdex) !=0 }
                fn set_bit(&mut self, bitdex:usize, val:bool) {
                    let mask = 1<<bitdex;
                    *self = (*self & !mask) | (val as Self)<<bitdex; //Clear bit then set it
                }

                fn ctz<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> usize {
                    ((!Self::bitmask(range)) | self).count_zeros() as usize //  111111BitsWeWant1111111 , others are 1
                }
                fn get_bits<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> Self {Self::bitmask(range) & self}
                fn popcnt<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&self, range:&R) -> usize {
                    self.get_bits(range).count_ones() as usize
                }
                fn set_these_bits<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&mut self, bits:Self, range:&R) {
                    //XOR is commutative and self-inverse
                    //A ^B ^B  = A ^(B^B), B^B = 0, So A^B^B = A , dobule xoring undos xor
                    //Here we Self^Bits and truncate it, then we xor it to reverse the xors, giving us self and truncated bits 
                    let diff = (*self ^ bits) & Self::bitmask(range); //Truncated diff
                    *self ^= diff; //XORing the diff undo the xor leaving just a truncated bits and self
                }
                fn set_all_bit(val:bool) -> Self {(0 as Self).wrapping_sub(val as Self) /*0000.. if 0  ,  1111.. if 1*/}
                fn set_bits<R:RangeBounds<usize>+ NumRangeExtract<usize>>(&mut self, range:&R, val:bool) {
                    self.set_these_bits(Self::set_all_bit(val),range)
                }
                fn first_set_bit(&self) -> usize {self.trailing_zeros() as usize} //Can go OOB
                fn last_set_bit(&self) -> usize {(Self::BITS -1 - self.leading_zeros()) as usize} //Can go OOB
                fn mut_bit(&mut self, bit:usize) -> MutBitProxy<'_,Self> {MutBitProxy::<Self>::new(self,bit)} //Returns proxy struct, on drop proxy updates bit
            }
        )*
    }
}
bittypes!(u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize);

use core::ops::{Bound,RangeBounds};

pub trait NumRangeExtract<T>: RangeBounds<T>  {
    fn end(&self) -> Option<T>;
    fn start(&self) -> Option<T>;
}

pub trait BitZRange<T>: NumRangeExtract<T> {
    fn bits_end(&self) -> usize;
    fn bits_start(&self) -> usize;
}

macro_rules! num_rangy {
    ($($type:ty),*) => {
        $(
            impl <R:RangeBounds<$type>>NumRangeExtract<$type> for R {
                fn end(&self) -> Option<$type> {
                    match self.end_bound() {
                        Bound::Included(val) => Some(*val),
                        Bound::Excluded(val) => Some((*val).saturating_sub(1)),
                        Bound::Unbounded =>  None
                    }
                }
                fn start(&self) -> Option<$type> {
                    match self.start_bound() {
                        Bound::Included(val) => Some(*val),
                        Bound::Excluded(val) => Some((*val).saturating_add(1)), // Rare in standard Rust ranges, but possible
                        Bound::Unbounded => None           // e.g., ..5 starts at index 0
                    }
                }
            }

            impl <R:NumRangeExtract<$type>> BitZRange<$type> for R {
                fn bits_start(&self) -> usize {self.start().unwrap_or(0).max(0) as usize}
                fn bits_end(&self) -> usize {self.end().unwrap_or(<$type>::BITS as $type).min((<$type>::BITS as $type)-1) as usize}
            }

        )*
    }
}
num_rangy!(u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize);