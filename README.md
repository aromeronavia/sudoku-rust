# sudoku-rust
Sudoku Implementation in Rust just for the lulz

The code just generates sudoku boards at the moment.

# Running the code
```sh
$ cargo run
```

# Running tests
```sh
➜  sudoku git:(main) ✗ cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/sudoku-df1eaeab5155eb19)

running 1 test
[1, 3, 7, 6, 9, 4, 5, 8, 2]
[8, 2, 5, 7, 3, 1, 6, 9, 4]
[9, 6, 4, 2, 5, 8, 3, 1, 7]
[7, 1, 8, 4, 6, 5, 2, 3, 9]
[6, 4, 9, 3, 2, 7, 8, 5, 1]
[2, 5, 3, 8, 1, 9, 4, 7, 6]
[4, 8, 2, 9, 7, 3, 1, 6, 5]
[3, 9, 1, 5, 4, 6, 7, 2, 8]
[5, 7, 6, 1, 8, 2, 9, 4, 3]
test sudoku_board_is_correct ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6
.99s

```
