use std::env;
use std::fs::File;
use std::io::Read;

fn tac(path: &String) {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}: {}", path, err);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(err) => {
            eprintln!("{}: {}", path, err);
            return;
        }
        _ => {}
    };

    for s in contents.rsplit("\n") {
        println!("{}", s);
    }
}

fn main() {
    for arg in env::args().skip(1) {
        tac(&arg);
    }
}
