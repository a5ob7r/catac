use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;

const BUF_SIZE: usize = 1024 * 4;

fn cat(path: &String) -> io::Result<()> {
    let mut file = File::open(path)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut buf = [0; BUF_SIZE];
    while let Ok(n) = file.read(&mut buf) {
        if n == 0 {
            break;
        };

        handle.write_all(&buf[..n])?;
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
