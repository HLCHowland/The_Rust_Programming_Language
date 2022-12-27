// Provide error handling for opening a file 
use std::fs::File; 
fn main() { 
    let f = File::open("hello.txt"); 
    let f = match f { 
        Ok(file) => file, 
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }, 
    }; 
}



use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}



