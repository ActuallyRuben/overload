#![feature(fn_traits, unboxed_closures)]
pub use overload_macros::overload;
use std::ops::Add;
#[cfg(test)]
mod test;

overload! {
    pub fn returns_one {
        <T: Add>(a: T, b: T) -> T::Output {
            let result = a + b;
            result
        }

        (a: i32) -> i32 {
            assert_eq!(a, 1, "Input must be one");
            a
        }

        () -> u64 {
            1
        }
    }
}