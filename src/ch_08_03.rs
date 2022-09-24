use std::collections::HashMap;

pub fn ch_08_03() {
    let mut hm = HashMap::new();
    hm.insert("foo", 3);
    hm.insert("foo", 10);
    hm.insert("bar", 4);
    for (key, value) in &hm {
        println!("{} {}", key, value);
    }
    let count = hm.entry("non-existent").or_insert(99);
    *count += 3;
    println!("{:?}", hm.entry("foo").or_insert(99));
    println!("{:?}", hm);
}
