# Input Rust
You know that Jail i compiling into Rust Code so we added a special keyword `__rust` that will add your provided code! Here some example code:
```
__rust "println!(`Hello, World!`)"
```
If you programming in rust you know that with **`** you can't create a string, but our compiler replace all **`** into **"**.
This what code will be generated:
```rust
/* Code from __rust keyword */
println!("Hello!");
/* Code end. */
```
So that all about `__rust` keyword in **Jail**!
