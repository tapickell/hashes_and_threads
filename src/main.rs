extern crate crypto_hash;
extern crate time;

use crypto_hash::{hex_digest, Algorithm};
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::thread;
use time::Time;

fn input(i: u8) -> Vec<u8> {
    let mut vec = vec![i];
    vec.resize(10000000, i);
    vec
}

fn hash_test(name: String) -> String {
    let started = Time::now();
    for n in 0..50 {
        let input = input(n);
        let result = hex_digest(Algorithm::SHA256, &input);
        let el = Time::now() - started;
        println!("{} {:?} sha {:?} :: {:?}", name, el, n, result);
    }
    let elapsed = Time::now() - started;
    return String::from(format!("{} SHA256 elapsed {:?}", name, elapsed));
}

fn thread_hash(n: i8, tx: mpsc::Sender<String>) {
    let handle = thread::spawn(move || {
        let resp = hash_test(String::from(format!("{} threaded", n)));
        tx.send(resp).unwrap();
        println!("{} threaded hash_test complete", n);
    });
}

fn threaded_channels() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx);
    let tx3 = mpsc::Sender::clone(&tx);

    let h1 = thread_hash(1, tx1);
    let h2 = thread_hash(2, tx2);
    let h3 = thread_hash(3, tx3);

    for recv in rx {
        println!("{}", recv);
    }
}

fn not_main() {
    threaded_channels();
    println!("all hash_test complete");
}

fn main() {
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(), false);
    assert_eq!(x.is_some(), true);

    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }
    assert_eq!(x, Some(42));

    println!("main complete");
}

fn string_slice() {
    let str = String::from("str");
    let st = &str[..2];
    let tr = &str[1..];
    println!("{}, {}", st, tr);
}

enum ListSingly {
    Cons(i32, Box<ListSingly>),
    Nil,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    fn set_next(&mut self, next: Node<T>) {
        self.next = Some(Box::new(next));
    }
}

fn singly_linked_list() {
    let mut head = Node {
        next: None,
        value: 1,
    };

    let next = Node {
        next: None,
        value: 2,
    };

    head.set_next(next);

    println!("{:?}", head);
}
