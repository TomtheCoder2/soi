use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

use text_io::read;

/// Reads a single line and converts it to a vector of type T
/// # Example
/// ```
/// use soi::read_vec;
/// let v: Vec<usize> = read_vec();
/// ```
/// # Panics
/// Panics if the input is not a valid vector of type T
pub fn read_vec<T>() -> Vec<T> where <T as FromStr>::Err: Debug, T: FromStr {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split(" ")
        .map(|x| x.to_string())
        // delete all empty strings
        .filter(|x| x != "")
        .collect::<Vec<String>>()
        .iter()
        .filter(|x| **x != " ".to_string())
        .map(|x| match x.parse::<T>() {
            Ok(x) => x,
            Err(e) => panic!("Error parsing input: {:?}", e),
        })
        .collect::<Vec<T>>()
}

pub fn read_vec_len<T>(len: usize) -> Vec<T> where <T as FromStr>::Err: Debug, T: FromStr + Display {
    let mut v = Vec::with_capacity(len);
    for _ in 0..len {
        let a = read!();
        v.push(a);
    }
    v
}