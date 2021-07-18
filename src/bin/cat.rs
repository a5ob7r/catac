use std::io;
use std::{env, fs::File, io::Read};

fn cat(path: &String) -> io::Result<()> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);

    return Ok(());
}

fn main() {
    for arg in env::args().skip(1) {
        if let Err(err) = cat(&arg) {
            eprintln!("{}: {}", arg, err);
        }
    }
}
