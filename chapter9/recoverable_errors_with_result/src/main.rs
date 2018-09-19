use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _f = File::open("hello.txt");
    let f = match _f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file {:?}", error)
        },
    };

    // Shortcut for Panic on Error: unwrap and expect
    let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}
