# Overview

This project is a Rust file encryption tool that can encrypt and decrypt files using a simple XOR cipher. I built it to get real experience writing Rust code and to learn how the language handles memory, file operations, and data safety. Creating this tool helped me understand how ownership and borrowing work in practice while building something that actually runs from the command line. It was also a good way to learn how to organize code with structs, functions, and error handling.

[Software Demo Video](https://youtu.be/lcGH1UArOoA)

---

# Development Environment

I wrote this program in Rust using Cargo to build and manage the project. I worked in Visual Studio Code with the Rust Analyzer extension, which made it easy to see compiler feedback and fix issues quickly. Everything was done with Rust’s standard library, so no outside libraries were needed. The focus was on learning the core language features and getting comfortable working with files, loops, and user input.

---

# Useful Websites

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Official Website](https://www.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

---

# Future Work

- Replace the XOR cipher with a more secure encryption algorithm such as AES.
- Add command-line arguments to allow encryption and decryption without interactive input.
- Implement persistent history tracking to log encrypted and decrypted files between runs.
