use std::iter::Iterator;
use std::option::Item;
use std::num::One;
use Integer;

pub trait ToBytes<'lt, BytesIterator:Iterator<u8> + 'lt>{
    fn to_bytes(& 'lt self) -> BytesIterator;
}

impl <'lt> ToBytes<'lt Item<u8>> for u8 {
    fn to_bytes(& 'lt self) -> Item<u8> {
        Some(*self).into_iter()
    }
}

pub struct U32Bytes {
    num:u32,
    pos:u8
}
impl Iterator<u8> for U32Bytes {
    fn next(& mut self) -> Option<u8> {
        if self.pos >= 4 {
            None
        } else {
            let res = (self.num & 0xff) as u8;
            self.num >>= 8;
            self.pos += 1;
            Some(res)
        }
    }
}

impl <'lt> ToBytes<'lt U32Bytes> for u32 {
    fn to_bytes(& 'lt self) -> U32Bytes {
        U32Bytes {num:*self, pos: 0}
    }
}

#[test]
fn test_u8_u32_bytes() {

    assert_eq!(72u8.to_bytes().collect::<Vec<u8>>(), vec![72u8]);
    assert_eq!(0x87654321u32.to_bytes().collect::<Vec<u8>>(), 
            vec![0x21u8, 0x43u8, 0x65u8, 0x87u8]);
}

pub fn mod_pow<'lt, BI:Iterator<u8> + 'lt, T:ToBytes<'lt, BI> + Integer>
(a: T, power: &'lt T, modulus: &T) -> T {
    let mut res:T = One::one();
    let mut a_p = a;
    for byte in power.to_bytes(){
        for bit_index in range(0, 8) {
            if (byte & (1 << bit_index)) != 0 {
                res = (res * a_p).mod_floor(modulus);
            }
            a_p = (a_p * a_p).mod_floor(modulus);
        }
    }
    res
}
#[test]
fn test_mod_pow() {
    assert_eq!(mod_pow(2u8, &2u8, &10u8), 4u8);
    assert_eq!(mod_pow(2u32, &10u32, &100u32), 24u32);
}
