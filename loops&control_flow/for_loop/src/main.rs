fn main() {
    for i in 1..10 {
        println!("i is {}", i);
    }

    println!("\n");

    for i in (1..=5).rev() {
        println!("i is {}", i);
    }

    println!("\n");

    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers {
        println!("number is {}", number);
    }
}
