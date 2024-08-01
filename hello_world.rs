#![allow(unused)]
use std::env;

fn main() {
    // Prints each argument on a separate line
    for argument in env::args_os() {
        println!("{:?}", argument);
    }
    for (key, value) in env::vars_os() {
    println!("{:?}: {:?}", key, value);
}
    println!("Hello, world!");
}
