## roaring_rust_benchmark

Basic benchmark to compare different Roaring bitmap implementations in Rust.

## Requirement

- You need the nightly version of rust (see https://doc.rust-lang.org/book/nightly-rust.html).
- A working C compiler.

## Usage

```bash
cargo bench
```

## Results

All tests executed on Linux systems with gcc version 5.3.0 and rustc 1.13.0-nightly (eac41469d 2016-08-30).

Intel Haswell processor (2013):

```bash
$ cargo bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/roaring_rust_benchmark-f8c8fa6f1c349de3

running 2 tests
test tests::croaring_create_bench ... bench:         227 ns/iter (+/- 0)
test tests::roaring_create_bench  ... bench:         249 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

     Running target/release/roaring_rust_benchmark-13eacb534ca2ee15

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

Intel Skylake processor (2015):

```bash
$ cargo bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/roaring_rust_benchmark-f8c8fa6f1c349de3

running 2 tests
test tests::croaring_create_bench ... bench:         211 ns/iter (+/- 1)
test tests::roaring_create_bench  ... bench:         231 ns/iter (+/- 2)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

     Running target/release/roaring_rust_benchmark-13eacb534ca2ee15

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

