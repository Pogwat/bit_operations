# bitops
Bit manipulation as fast as possible
#![no_std] compatible

## Common Examples 
Note: bits are 0 indexed

Set a specific bit in a type
```rust
use bit_operations::BitOps;
let mut num:u8 =0;
num.set_bit(0,true);
assert_eq!(num, 1_u8);
```

Get a specific bit in a type
```rust
use bit_operations::BitOps;
let mut num:u8 = 5; //0b101
assert_eq!(true, num.get_bit(2));
//Chain with set bit
num.set_bit(5,true);
assert_eq!(true, num.get_bit(5));
assert_eq!(0b00100101_u8, num); //NOTE: rust uses msb to lsb binary notation
```

Get a bitmask for a specfic type
```rust
use bit_operations::BitOps;
let mask:u8 = u8::bitmask(&(0..=7));
let num:u8 = u8::MAX;
assert_eq!(num & !mask, 0);
```

Get Number of 1's or 0's in a type 
```rust
use bit_operations::BitOps;
let num:u8 = 0b01010111;
assert_eq!(num.ctz(&(0..=7)),3);
assert_eq!(num.popcnt(&(0..=7)),5);
```

Set bits of a type to those of another
```rust
use bit_operations::BitOps;
let mut num:u8     = 0b10011111;
let bits_to_set:u8 = 0b01010000;
num.set_these_bits(bits_to_set,&(0..=6));
assert_eq!(   num,   0b11010000);
```

Set bits of type to a bool
```rust 
use bit_operations::BitOps;
let mut num:u8 = 0b11111111;
num.set_bits(&(1..=5),false);
assert_eq!(num, 0b11000001);
```

Get Mutable Refrence to Bit. IMPORTANT: ref needs to be dropped for bit to update
```rust
use bit_operations::BitOps;
let mut num:u8 =0;
{ 
    let mut bit_mut = num.mut_bit(7);
    *bit_mut =true;
} //MUT REF MUST BE DROPPED FOR BIT TO BE UPDATED!!! DROP UPDATES
assert_eq!(num.get_bit(7), true);
assert_eq!(num,0b10000000);
```

Get first or last set or unset bit in a specific type
```rust
    use bit_operations::BitOps;
    let num:u8 = 0b10110100;
    // first/last set bit
    assert_eq!(num.first_one(&(0..)), Some(2));
    assert_eq!(num.last_one(&(0..)), Some(7));
    
    // first/last unset bit
    assert_eq!(num.first_zero(&(0..)), Some(0));
    assert_eq!(num.last_zero(&(0..)), Some(6));

    //using a custom range
    assert_eq!(num.first_one(&(0..=1)), None);
    
```

for full documentation use docs.rs: [docs](https://docs.rs/bit_operations/latest/bit_operations/)
