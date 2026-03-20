fn main() {
    let sentence1 = String::from("Hello, world!");
    let sentence2 = "Hello, world!".to_string();
    let slice1 = &"Hello, world!"[0..3];
    println!("{}", slice1);
    let mut sentence3 = String::new();
    sentence3.push_str("Hello, world!");

    println!("Sentence 1: {}", sentence1);
    println!("Sentence 2: {}", sentence2);
    println!("Sentence 3: {}", sentence3);

    let combined = format!("{} {}", sentence1, sentence2);
    println!("Combined: {}", combined);

    let words: Vec<&str> = sentence1.split_whitespace().collect();
    println!("Words in sentence1: {:?}", words);

    for c in sentence1.chars() {
        println!("Character: {}", c);
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("{} is a vowel", c),
            _ => println!("{} is a consonant", c),
        }
    }

    for i in 0..100 {
        println!("{}", i);
    }
}
