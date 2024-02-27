
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let _e = CustomSmartPointer {
        data: String::from("some data to drop early"),
    };
    println!("CustomSmartPointer created.");
    drop(_e);
    println!("CustomSmartPointer dropped before the end of main.");

    let _a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&_a));
    let _b = Cons(3, Rc::clone(&_a));
    println!("count after creating b = {}", Rc::strong_count(&_a));
    {
        let _c = Cons(4, Rc::clone(&_a));
        println!("count after creating c = {}", Rc::strong_count(&_a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&_a));
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


    