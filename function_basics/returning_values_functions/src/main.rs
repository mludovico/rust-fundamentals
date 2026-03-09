use std::process::ExitCode;

fn main() -> ExitCode {
    let chunk = split_string("Hello, world!".to_string(), ',', 1);
    println!("Split string {}", chunk);
    ExitCode::FAILURE
}

fn split_string(input: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = input.split(delimiter).collect();
    let result: Option<&&str> = parts.get(field);

    result.expect("ooops! something went wrong here...").to_string()
}
