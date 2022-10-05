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

pub fn ch_15_01() {
    basic_box();
    recurse_box();
    deref_examples();
}
