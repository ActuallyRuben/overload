use std::ops::{Add, Mul};
use std::fmt::Display;
use std::collections::VecDeque;
use std::io::Write;
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

overload! {
    fn print_all {
        <W: Write, T: Display>(mut output: W, v: &Vec<T>) -> std::io::Result<()> {
            for x in v{
                writeln!(output, "{}", x)?;
            }
            Ok(())
        }

        <W: Write, T: Display>(mut output: W, mut v: VecDeque<T>) -> std::io::Result<()> {
            while !v.is_empty() {
                writeln!(output, "{}", v.pop_front().unwrap())?;
            }
            Ok(())
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

#[test]
fn printing() {
    let mut write_buffer: Vec<u8> = Vec::new();
    print_all(&mut write_buffer, &vec![1, 2, 3, 4]).unwrap();
    print_all(&mut write_buffer, VecDeque::from([5, 6, 7, 8])).unwrap();
    let result = String::from_utf8(write_buffer).unwrap();
    assert_eq!(result, "1\n2\n3\n4\n5\n6\n7\n8\n");
}