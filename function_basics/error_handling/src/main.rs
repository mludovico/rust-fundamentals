use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

fn main() {
    println!("Enter the path to the file you want to read:");

    let mut path = String::new();
    std::io::stdin()
        .read_line(&mut path)
        .expect("Must be a valid path");
    println!(
        "Looking for a file at {}",
        std::env::current_dir().unwrap().display()
    );
    let file = File::open(path.trim());
    match file {
        Ok(file) => read_and_print(file),
        Err(error) => handle_error(error),
    };
}

fn read_and_print(file: File) {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn handle_error(error: std::io::Error) {
    match error.kind() {
        ErrorKind::NotFound => {
            println!(
                "File not found. Please create the file and try again. error: {:?}",
                error.to_string()
            );
        }
        ErrorKind::PermissionDenied => {
            println!(
                "Permission denied. Please check your permissions and try again. error: {:?}",
                error.to_string()
            );
        }
        _ => {
            println!("Problem opening the file: {:?}", error);
        }
    }
}
