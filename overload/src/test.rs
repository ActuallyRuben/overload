use std::ops::{Add, Mul};

use crate::overload;

overload! {
    pub fn add {
        (a: i32, b: i32) -> i32 {
            let result = a + b;
            result
        }

        ((a, b): (i32, i32), (c, d): (i32, i32)) -> (i32, i32) {
            (a + c, b + d)
        }

        (a: &str, b: &str) -> Option<i32> {
            Some(a.parse::<i32>().ok()? + b.parse::<i32>().ok()?)
        }

        <I: Iterator<Item=T>, T: Add<T, Output=T> + Default>(vals: I) -> T {
            vals.fold(Default::default(), |x, y| {
                x + y
            })
        }

        () -> i32 {
            0
        }
    }
}

overload! {
    fn double {
        <T: Mul<u32>>(value: T) -> T::Output {
            value * 2
        }

        (a: &str, _: ()) -> String {
            format!("{}{}", a, a)
        }
    }
}

#[test]
fn addition() {
    assert_eq!(add(6, -7), -1);
    assert_eq!(add((4, 5), (6, 7)), (10, 12));
    assert_eq!(add("6", "-7"), Some(-1));
    assert_eq!(add([5, 6, 7, 8].into_iter()), 5 + 6 + 7 + 8);
    assert_eq!(add(), 0);
}

#[test]
fn doubling() {
    assert_eq!(double(5), 10);
    assert_eq!(double("hello", ()), "hellohello");
}