fn main() {
    println!("Please enter a greeting (Hello, Hi, Goodbye):");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read input");
    match name.trim() {
        "Hello" => println!("Hello, world!"),
        "Hi" => println!("Hi, world!"),
        "Goodbye" => println!("Goodbye, world!"),
        _ => println!("Unknown greeting!"),
    }

    println!("Please enter a number:");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read input");
    let number: i32 = number.trim().parse().expect("Please enter a valid number");
    match number {
        0 => println!("Zero"),
        1..=10 => println!("Between 1 and 10"),
        11..=100 => println!("Between 11 and 100"),
        _ => println!("Greater than 100 or negative"),
    }
}
