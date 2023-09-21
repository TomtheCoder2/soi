# SOI

Provides simple io for reading and writing from stdin and stdout.

## Usage

```rust
use soi_io::{read, read_vec, read_vec_len};

fn main() {
    // reads the first int from stdin
    let n: i32 = read!();
    // reads the next 3 ints from stdin (on the same line)
    let v: Vec<usize> = read_vec_len(3);
    // reads the rest of the line as ints
    let v2: Vec<usize> = read_vec();
    println!("n: {}, v: {:?}, v2: {:?}", n, v, v2);
}
```
Example input:
```text
1 2 3 4 5 6 7 8 9
```
Example output:
```text
n: 1, v: [2, 3, 4], v2: [5, 6, 7, 8, 9]
```
