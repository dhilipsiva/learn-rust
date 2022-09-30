fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn ch_10_03() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("efdddd");
        result = longest(&string1, &string2);
    }
    println!("The longest string is {}", result);
}
