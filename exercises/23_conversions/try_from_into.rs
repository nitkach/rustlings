// `TryFrom` is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as `From`. The main
// difference is that this should return a `Result` type instead of the target
// type itself. You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html

#![allow(clippy::useless_vec)]
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for the `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// TODO: Tuple implementation.
// Correct RGB color values must be integers in the 0..=255 range.
impl TryFrom<(i16, i16, i16)> for Color {
    // where
    // u8: std::convert::TryFrom<T>,
    // {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        match (
            u8::try_from(tuple.0),
            u8::try_from(tuple.1),
            u8::try_from(tuple.2),
        ) {
            (Ok(red), Ok(green), Ok(blue)) => Ok(Self { red, green, blue }),
            _ => Err(IntoColorError::IntConversion),
        }
    }
}

// TODO: Array implementation.
impl<T> TryFrom<[T; 3]> for Color
where
    u8: std::convert::TryFrom<T>,
{
    type Error = IntoColorError;

    fn try_from(arr: [T; 3]) -> Result<Self, Self::Error> {
        let mut values = arr.into_iter();
        let (Some(Ok(red)), Some(Ok(green)), Some(Ok(blue))) = (
            values.next().map(u8::try_from),
            values.next().map(u8::try_from),
            values.next().map(u8::try_from),
        ) else {
            return Err(IntoColorError::IntConversion);
        };

        Ok(Self { red, green, blue })
    }
}

// TODO: Slice implementation.
// This implementation needs to check the slice length.
impl<T: Clone + Copy> TryFrom<&[T]> for Color
where
    u8: std::convert::TryFrom<T>,
{
    type Error = IntoColorError;

    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        let mut values = slice.iter().copied();
        let (Some(Ok(red)), Some(Ok(green)), Some(Ok(blue)), None) = (
            values.next().map(u8::try_from),
            values.next().map(u8::try_from),
            values.next().map(u8::try_from),
            values.next(),
        ) else {
            return Err(IntoColorError::IntConversion);
        };

        Ok(Self { red, green, blue })
    }
}

fn main() {
    // Using the `try_from` function.
    let c1 = Color::try_from((183, 65, 14));
    println!("{c1:?}");

    // Since `TryFrom` is implemented for `Color`, we can use `TryInto`.
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{c2:?}");

    let v = vec![183, 65, 14];
    // With slice we should use the `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{c3:?}");
    // or put the slice within round brackets and use `try_into`.
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{c4:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use IntoColorError::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(Color::try_from((256, 1000, 10000)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(Color::try_from((-1, -10, -256)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(Color::try_from((-1, 255, 255)), Err(IntConversion));
    }

    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntConversion));
    }

    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14,
            }
        );
    }

    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }

    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(BadLen));
    }
}
