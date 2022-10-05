use std::ops::Deref;

fn basic_box() {
    let b = Box::new(5);
    dbg!(&b);
    // uncommenting the following line will throw an error because: b == Box::new(5)
    // if b == 5 {
    //    println!("`b` is 5")
    // }
    let i = *b; // dereferencing
    assert_eq!(i, 5);
    dbg!(&b);

    let b = Box::new(String::from("my box"));
    let s = *b; // dereferencing
    assert_eq!(s, String::from("my box"));

    // uncommenting the following will fail becuase of move; does not implement Copy trait
    // dbg!(&b);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn recurse_box() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    dbg!(&list);
}

fn deref_examples() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping T");
    }
}
fn ch_15_02() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let z = MyBox::new(x);
    drop(z);
    println!("z already dropped!");
    println!("y not dropped yet!");
}

pub fn ch_15_01() {
    basic_box();
    recurse_box();
    deref_examples();
    ch_15_02();
}
