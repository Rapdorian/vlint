//! Variable length integer encoding/decoding helper

use num::{FromPrimitive, PrimInt, ToPrimitive, Zero};

/// Provides methods from converting to/from a variable length encoding
///
/// Uses the high bit of each byte to indicate if the encoding continues.
/// high bit is set if the encoding continues to the next byte
pub trait VLInt {
    /// get the variable length encoding of this integer
    fn vlint(self) -> Vec<u8>;

    /// parse a variable length encoding into an integer
    fn from_vlint(bytes: &[u8]) -> Self;
}

impl<I> VLInt for I
where
    I: PrimInt + FromPrimitive + Zero + ToPrimitive,
{
    fn vlint(mut self) -> Vec<u8> {
        let mut res = vec![];

        loop {
            let chunk = (self & (I::from_u8(0b1111111).unwrap())).to_u8().unwrap();
            self = self >> 7;
            let cont = if self > I::zero() { 1 } else { 0 } as u8;
            res.push(cont << 7 | chunk);

            if self == I::zero() {
                break;
            }
        }
        res
    }

    fn from_vlint(bytes: &[u8]) -> Self {
        let mut res = I::zero();

        for (i, byte) in bytes.iter().enumerate() {
            res = res | (I::from_u8(byte & 0b1111111).unwrap() << (i * 7));
            if *byte <= 127 {
                // if high bit is not set
                return res;
            }
        }
        panic!("vlint never terminates");
    }
}

#[cfg(test)]
mod tests {
    use super::VLInt;
    #[test]
    fn to_vlint() {
        assert_eq!(4.vlint(), vec![0b000_0100]);
        assert_eq!(0.vlint(), vec![0b000_0000]);
        assert_eq!(127.vlint(), vec![0b0111_1111]);
        assert_eq!(300.vlint(), vec![0b1010_1100, 0b0000_0010]);
    }

    #[test]
    fn roundtrip() {
        assert_eq!(4, i32::from_vlint(&4.vlint()));
        assert_eq!(0, i32::from_vlint(&0.vlint()));
        assert_eq!(127, i32::from_vlint(&127.vlint()));
        assert_eq!(300, i32::from_vlint(&300.vlint()));
    }
}
