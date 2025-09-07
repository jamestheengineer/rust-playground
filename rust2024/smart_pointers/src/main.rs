enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}