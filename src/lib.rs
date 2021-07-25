//! # ananas
//!
//! Convert bytes into NaN payloads!
//!

const SGNBIT: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
const NANPRE: u32 = 0b0111_1111_1000_0000_0000_0000_0000_0000;
const SIGNAL: u32 = 0b0000_0000_0100_0000_0000_0000_0000_0000;
const TWOBYT: u32 = 0b0000_0000_0011_0000_0000_0000_0000_0000;
const ONEBYT: u32 = 0b0000_0000_0010_0000_0000_0000_0000_0000;
const PAYLD1: u32 = 0b0000_0010_0000_0000_1111_1111_0000_0000;
const PAYLD2: u32 = 0b0000_0010_0000_0000_0000_0000_1111_1111;

/// Store a u16 into a f32 NaN.
pub fn u16_to_nan(inp: u16) -> f32 {
    f32::from_bits(NANPRE | TWOBYT | (inp as u32))
}

/// Extract a u16 from a f32 NaN.
pub fn nan_to_u16(inp: f32) -> Option<u16> {
    let (b1, b2) = from_f32(inp);
    match b1 {
        Some(x) => match b2 {
            Some(y) => Some((x as u16) | ((y as u16) << 8)),
            None => Some(x as u16),
        },
        None => None,
    }
}

/// Convert a slice of 1 or 2 bytes into a f32 NaN.
fn into_f32<const N: usize>(inp: &[u8; N]) -> f32 {
    match N {
        2 => f32::from_bits(NANPRE | TWOBYT | ((inp[0] as u32) << 8) | (inp[1] as u32)),
        1 => f32::from_bits(NANPRE | ONEBYT | ((inp[0] as u32) << 8)),
        _ => panic!("Only 1 or 2 bytes can be put in a f32"),
    }
}

/// Convert a f32 NaN into up to two bytes.
fn from_f32(inp: f32) -> (Option<u8>, Option<u8>) {
    let x = inp.to_bits();
    if (x & NANPRE) != NANPRE {
        (None, None)
    } else {
        match x & TWOBYT {
            TWOBYT => (Some(((x & PAYLD1) >> 8) as u8), Some((x & PAYLD2) as u8)),
            ONEBYT => (Some(((x & PAYLD1) >> 8) as u8), None),
            _ => (None, None),
        }
    }
}

/// Convert a slice of bytes into a vec of NaNs.
///
pub fn bytes_to_nan(s: &[u8]) -> Vec<f32> {
    let mut out = vec![];
    for chunk in s.chunks(2) {
        let first = chunk.first().unwrap();
        let second = chunk.get(1);
        match second {
            Some(l) => out.push(into_f32(&[*first, *l])),
            None => out.push(into_f32(&[*first])),
        }
    }
    out
}

/// Convert a slice of f32 into a vec of bytes.
///
/// This function skips anything that is not an encoded NaN.
pub fn nan_to_bytes(s: &[f32]) -> Vec<u8> {
    let mut out = vec![];
    for el in s.iter() {
        let (first, second) = from_f32(*el);
        if let Some(i) = first {
            out.push(i)
        }
        if let Some(i) = second {
            out.push(i)
        }
    }
    out
}

/// Convert a string to a vec of NaNs.
pub fn str_to_nan(s: &str) -> Vec<f32> {
    bytes_to_nan(s.as_bytes())
}

/// Convert a vec of NaNs into a string.
pub fn nan_to_str(s: &[f32]) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(nan_to_bytes(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2into() {
        let a: u8 = 0b0101_0100;
        let b: u8 = 0b0001_0111;
        assert!(into_f32(&[a, b]).is_nan());

        let (a2, b2) = from_f32(into_f32(&[a, b]));
        assert_eq!(a2.unwrap(), a);
        assert_eq!(b2.unwrap(), b);
    }

    #[test]
    fn test_1into() {
        let a: u8 = 0b0101_0100;
        assert!(into_f32(&[a]).is_nan());

        let (a2, b2) = from_f32(into_f32(&[a]));
        assert_eq!(a2.unwrap(), a);
        assert_eq!(b2, None);
    }

    #[test]
    fn test_nanvec() {
        let a = "hello world!";
        let mut b = bytes_to_nan(a.as_bytes());
        assert!(b[0].is_nan());

        b.insert(3, 3303.3);
        let c = nan_to_bytes(&b);
        let c = String::from_utf8(c).unwrap();
        assert_eq!(c, a);
    }

    fn test_u16() {
        let a: u16 = 0b1011_1111_0101_0100;
        let b = u16_to_nan(a);
        assert!(b.is_nan());
        let c = nan_to_u16(b).unwrap();
        assert_eq!(c, a);
    }
}
