use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn")
    let f = File::open("toto.txt");

    // match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("problem opening the file: {:?}", other_error),
    //     },
    // };

    //alternative
    f.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("problem opening the file: {:?}", error)
        }
    });
}
