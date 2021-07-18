use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn tac(path: &String) -> io::Result<()> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for s in contents.rsplit("\n") {
        println!("{}", s);
    }

    return Ok(());
}

fn main() {
    for arg in env::args().skip(1) {
        if let Err(err) = tac(&arg) {
            eprintln!("{}: {}", arg, err);
        }
    }
}
