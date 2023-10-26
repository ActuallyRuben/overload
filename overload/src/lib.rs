#![feature(fn_traits, unboxed_closures)]
#![feature(tuple_trait)]

use std::marker::Tuple;
pub use overload_macros::overload;

#[cfg(test)]
mod test;

trait CheekyFnOnce<R, Ts: Tuple>: FnOnce<Ts, Output=R> {
    fn call(&self, args: Ts) -> R;
}
