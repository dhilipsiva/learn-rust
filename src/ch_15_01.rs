fn basic_box() {
    let b = Box::new(5);
    dbg!(&b);
    // uncommenting the following line will throw an error because: b == Box::new(5)
    // if b == 5 {
    //    println!("`b` is 5")
    // }
    let i = *b; // dereferencing
    if i == 5 {
        println!("`b` is 5")
    }
    dbg!(&b);

    let b = Box::new(String::from("my box"));
    let s = *b; // dereferencing
    if s == String::from("my box") {
        println!("`s` is {s}")
    }
    // uncommenting the following will fail becuase of move; does not implement Copy trait
    // dbg!(&b);
}

pub fn ch_15_01() {
    basic_box();
}