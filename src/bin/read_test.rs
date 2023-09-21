use soi::{read, read_vec, read_vec_len};

fn main() {
    let n: i32 = read!();
    let v: Vec<usize> = read_vec_len(3);
    let v2: Vec<usize> = read_vec();
    println!("n: {}, v: {:?}, v2: {:?}", n, v, v2);
}