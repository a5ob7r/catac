use std::{env, fs::File, io::Read};

fn cat(path: &String) {
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

    println!("{}", contents);
}

fn main() {
    for arg in env::args() {
        cat(&arg);
    }
}
