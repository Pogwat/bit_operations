# bitops
Bit manipulation as fast as possible

## Common Examples 
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

for full documentation use docs.rs: [docs](https://docs.rs/bit_operations/latest/bit_operations/)