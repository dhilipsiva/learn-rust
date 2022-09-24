pub fn ch_08_01() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    for i in &mut v {
        *i += 10;
        println!("{}", i);
    }

    let m = &v[1];
    println!("{} {:?}", m, v.get(10).unwrap_or(&mut 4));
}
