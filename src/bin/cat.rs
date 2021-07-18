use std::io;
use std::process;
use std::str;
use std::{env, fs::File, io::Read};

const BUF_SIZE: usize = 1024 * 4;

// TODO: Really valid size? How treats the case if intermediate of valid UTF-8 sequence is on
// `BUF_SIZE` times index position on file?.
fn cat(path: &String) -> io::Result<()> {
    let mut file = File::open(path)?;

    let mut buf = [0; BUF_SIZE];
    while let Ok(n) = file.read(&mut buf) {
        if n == 0 {
            break;
        };

        match str::from_utf8(&buf[..n]) {
            Ok(s) => print!("{}", s),
            Err(err) => {
                return Err(io::Error::new(io::ErrorKind::Other, err));
            }
        }
    }

    return Ok(());
}

fn main() {
    for arg in env::args().skip(1) {
        if let Err(err) = cat(&arg) {
            eprintln!("{}: {}", arg, err);
            process::exit(1);
        }
    }
}
