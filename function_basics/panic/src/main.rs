fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    loop_and_panic(numbers);
    let my_string = String::from("Hello, Rust!");
    let random_char = get_random_index(&my_string);
    println!("Random character from string: {}", random_char);
    println!("Hello, world!");

}

fn loop_and_panic(numbers: Vec<i32>) {
    for number in numbers {
        if number < 0 {
            panic!("Negative number found: {}", number);
        }
        println!("Number: {}", number);
    }
}

fn get_random_index(str: &String) -> char {
    let idx = rand::random::<i32>();
    println!("Getting random index: {} from {}", idx, str);
    str.chars().nth(idx as usize).expect("index not found")
}
