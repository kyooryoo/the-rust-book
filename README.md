# Rust Language [Book](https://doc.rust-lang.org/book/title-page.html)
From the official document

## Get Started
Download rustup for installation:
`$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
Check version:
`$ rustc --version`
Check local document:
`$ rustup doc`
Update Rust:
`$ rustup update`
Unistall Rust:
`$ rustup self uninstall`

### Hello World
Recommend creating project specific directory.
Create *main.rs* source file with following code:
```
fn main() {
    println!("Hello, world!");
}
```
Compile the source file and run the executable:
`$ rustc main.rs`
`$ ./main`
`Hello, world!`
Using *cargo* is recommended for large projects.

### Hello Cargo
The same *cargo* works the same way over OSs.
Check the cargo readiness by its version:
`$ cargo --version`
Create a new project:
`$ cargo new hello_cargo`
`$ cd hello_cargo`

There are following items created by cargo:
* Cargo.toml: project configuration file
* Git related files such as *.gitignore*
* *src* folder which holds all source files
More items will be created when compiled

Cargo also provides the following commands:
* cargo build: compile and create executable
+ cargo put executable in *./target/debug*
* cargo run: build and run the program
* cargo check: check errors but not build
+ cargo check runs much faster without building
* cargo doc --open: create and open project doc

For releasing executable to end users:
* By default, `cargo build` is for development
* Run `cargo build --release` for production
* It creates an executable in *target/release*
* It takes more time on optimizing compiling
* The executable has the best running performance

Advantage of using cargo as the convention
* Facilitate checking, building, and running.
* Use the same cargo commands on different OSs.
* Manage intricate project with best practicel