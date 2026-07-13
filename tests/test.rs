use bit_operations::BitOps;
#[test]
fn bitops_get_set() {
    let mut num:u8 = 8; //0001
    println!("{}",num.get_bit(3));
    assert_eq!(num.get_bit(0), false);
    assert_eq!(num.get_bit(1), false);
    assert_eq!(num.get_bit(2), false);
    assert_eq!(num.get_bit(3), true);

    num.set_bit(7, true);
    println!("{}",num.get_bit(7));
    assert_eq!(num, (8+2_u8.pow(7)));
    assert_eq!(num.get_bit(7), true);
    assert_eq!(num.get_bit(3), true);
    println!("{}",num);
}

#[test]
fn bitops_bitmask() {
    let num:u8 =BitOps::bitmask(&(0..8));
    println!("bitmask:{:?}",num);
    assert_eq!(num, u8::MAX)
}

#[test]
fn bitops_popcnt_ctz() {
    let num:u8 = u8::MAX;
    println!("{}", num.ctz(&(0..8)));
    assert_eq!(num.count_zeros() as u8,num.ctz(&(0..8)));
    assert_eq!(num.count_ones() as u8,num.popcnt(&(0..8)));

    let num:u8 = 2_u8.pow(7)-1;
    assert_eq!(num.count_zeros() as u8,num.ctz(&(0..=7)));
    assert_eq!(num.count_ones() as u8,num.popcnt(&(0..=7)));

    assert_eq!(num.count_zeros() as u8,num.ctz(&(0..)));
    assert_eq!(num.count_zeros() as u8,num.ctz(&(..8)));
}

#[test]
fn bitops_get_set_bits(){
    let mut num: u8 = 0;
    num.set_bits(&(0..=2),true);
    println!("num: {}",num);
    assert_eq!(num, 2_u8.pow(3)-1);
    assert_eq!(num, num.get_bits(&(0..=2)));
}

#[test]
fn bitops_first_last_set_bit() {
    let bit_set = 5;
    let mut num:u8 = 2_u8.pow(bit_set); // 001
    assert_eq!(num.first_set_bit(), 5);
    assert_eq!(num.last_set_bit(), 5);
    let last_bit = 7;
    num += 2_u8.pow(last_bit);
    assert_eq!(num.last_set_bit(), 7);

}
#[test]
fn bitops_set_these_bits() {
    let mut num:u8 = 255;
    num.set_these_bits(0b0111, &(0..4));
    println!("{:08b}", num);
    assert_eq!(num, 255-2_u8.pow(3))
}
#[test]
fn get_mut_bitops() {
    let mut num:u8 =0b11111111;
    { let mut bit_mut = num.mut_bit(5);
    *bit_mut =false;} //MUT REF MUST BE DROPPED FOR BIT TO BE UPDATED!!! DROP UPDATES
    assert_eq!(num.get_bit(5), false);
    assert_eq!(num,0b11011111);
}

#[test]
fn type_bits() {
    assert_eq!(u8::TYPE_BITS, 8);
    assert_eq!(u8::BIT_BITS, 3);
}
