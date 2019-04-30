*** Interface is subject to a lot of experimentation, right now ***


## sufsort-rs

libdivsufsort bindings for Rust.

## Building sufsort-rs
Apart from a C compiler and Rust, you'll need to install _CMake_. Then run:

```sh
$ git clone https://github.com/srirampc/sufsort-rs
$ cargo build
```

## Example Usage
### Construction of Suffix array
```rust
extern crate sufsort_rs;
use self::sufsort_rs::SA;
let s = ("MISSISSIPPI").to_string();
let sax = s.construct_sa();

```

### Construction of Burrows Wheeler Transform
```rust
extern crate sufsort_rs;
use self::sufsort_rs::SA;
let s = ("MISSISSIPPI").to_string();
let bwt = s.construct_bwt();

```

### Pattern Search
```rust
extern crate sufsort_rs;
use self::sufsort_rs::SearchSA;
let s = ("MISSISSIPPI").to_string();
let sax: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
let pat = ("IS").to_string();
let rst = s.search_sa(&sax, &pat);
```

## Licence

`sufsort-rs` is primarily distributed under the terms of both the MIT license.

libdivsufsort is distributed under MIT License. See libdivsufsort/README.md.
