use std::fs::File;

fn main() {
    let _f = File::open("hello.txt");
    let f = match _f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file {:?}", error);
        },
    };
}
