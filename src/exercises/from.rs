// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.
#[allow(unused)]
use crate::exercises::trait_bounds::min;

pub struct WrappingU32 {
    inner: u32,
}

// impl<T, U> Into for WrappingU32
// where
// 	U: From
// {

// }

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { inner: value }
    }
}

#[test]
fn example() {
    let wrapping: WrappingU32 = 42.into();
    min(wrapping.inner, wrapping.inner);

    let wrapping = WrappingU32::from(42);

    min(wrapping.inner, wrapping.inner);
}
