use std::num::Saturating;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

// Macro for implementing From conversions
macro_rules! impl_from {
    ($from_type:ty) => {
        impl From<$from_type> for SaturatingU16 {
            fn from(value: $from_type) -> Self {
                SaturatingU16::new(value as u16)
            }
        }
    };
    ($from_type:ty, deref) => {
        impl From<$from_type> for SaturatingU16 {
            fn from(value: $from_type) -> Self {
                SaturatingU16::new(*value as u16)
            }
        }
    };
}

// Implement all From conversions
impl_from!(u16);
impl_from!(u8);
impl_from!(&u16, deref);
impl_from!(&u8, deref);

fn main() {
    // Example usage
    let a = SaturatingU16::new(100);
    let b = SaturatingU16::new(200);
    
    let sum = a + b;
    println!("Sum: {:?}", sum); // Should print: Sum: SaturatingU16 { value: 300 }

    let c: SaturatingU16 = 150u16.into();
    let d: SaturatingU16 = 50u8.into();
    
    let sum2 = c + d;
    println!("Sum2: {:?}", sum2); // Should print: Sum2: SaturatingU16 { value: 200 }
}