use std::fs::File;
use std::io;
use std::io::Read;

fn read_from_file() -> Result<String, io::Error> {
    let mut file = File::open("Cargo.toml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn ch_09_01() {
    let content = read_from_file().unwrap(); //.expect("Could not read the contents of the file");
    println!("{:?}", content);
}
