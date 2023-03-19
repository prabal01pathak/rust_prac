1. Cargo to compile and run the rust program: ` cargo run `
2. Rustc to compile the code: ` rustc ./src/main.rs`

<!-- - [x] 1. Cargo to compile and run the rust program: ` cargo run `
- [x] 2. Rustc to compile the code: ` rustc ./src/main.rs` -->
- [x] 1. what does unwrap do?
 - unwrap() returns the value of an Option<T> or panics if the value is None.
- [x] 2. what is Option<T>?
- Option<T> is an enum that can be either Some or None.
- [x] 3. what Some and None?
-  Some is a variant of Option<T> that contains a value of type T.
-  None is a variant of Option<T> that does not contain a value.
- [x] 4. what is enum?
- enum is a type that can be one of a few different variants.
- [x] 5. what is T in Option<T>?
- T is a generic type parameter.
- [x] 6. '' is of type char.
- [x] 7. "" is of type String.


# complie and build release
```sh
cargo build --release
```