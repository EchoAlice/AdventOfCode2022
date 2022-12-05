use std::io;
use std::io::{BufRead, BufReader, ErrorKind};
use std::fs;
use std::fs::File;
use std::path::Path;

/*
Error Notes
-----------

Result has 2 varients: Ok and Err
    enum Result<T, E> {
        Ok(T),            T represents the data typeof the value returns and E
        Err(E),           the type of error
    }
*/

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn bytes_to_string(bytes: Vec<u8>) -> String {
    let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
    println!("{}", s);
    return s
}

// Reading from files
pub fn read_to_bytes(path: &str) -> Vec<u8> {
    let data = fs::read(path).expect("Unable to read file");
    return data
}

// Leo's read lines
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}