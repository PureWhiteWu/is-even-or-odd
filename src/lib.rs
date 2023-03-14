use is_even::IsEven;
use is_odd::IsOdd;

pub trait IsEvenOrOdd {
    fn is_even_or_odd(&self) -> bool;
}

macro_rules! prim_impl {
    ($($t:tt)*) => {
        $(
            impl IsEvenOrOdd for $t {
                fn is_even_or_odd(&self) -> bool {
                    self.is_even() || self.is_odd()
                }
            }
        )*
    };
}

prim_impl!(i8 u8 i16 u16 i32 u32 i64 u64);
