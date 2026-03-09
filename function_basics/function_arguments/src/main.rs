fn main() {
    let mut numbers = Vec::new();
    let input = std::io::stdin();
    println!("Enter numbers to sum (type 'done' to finish):");
    loop {
        let mut buffer = String::new();
        input.read_line(&mut buffer).expect("Failed to read line");
        let trimmed = buffer.trim();
        if trimmed.eq_ignore_ascii_case("done") {
            break;
        }
        match trimmed.parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => println!("Please enter a valid number or 'done' to finish."),
        }
    }
    let sum = sum(&numbers);
    println!("The sum of the numbers is: {}", sum);
}

fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for &number in numbers {
        total += number;
    }
    total
}
