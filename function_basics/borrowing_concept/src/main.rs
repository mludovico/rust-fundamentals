fn main() {
    let my_integer = 42;
    let my_string = String::from("Hello, Rust!");
    let my_borrowed_string = my_string.clone();
    let my_vector = vec![1, 2, 3, 4, 5];
    let mut my_borrowed_vector = my_vector.clone();

    own_integer(my_integer);
    own_string(my_string);
    own_vector(my_vector);
    borrow_string(&my_borrowed_string);
    borrow_vector(&mut my_borrowed_vector);

    println!("{}", my_integer);
    // println!("{}", my_string);
    // println!("{:?}", my_vector);
    println!("{}", my_borrowed_string);
}

fn own_integer(n: i32) {
    println!("I own the integer: {}", n);
}

fn own_string(s: String) {
    println!("I own the string: {}", s);
}

fn own_vector(v: Vec<i32>) {
    println!("I own the vector: {:?}", v);
}

fn borrow_string(mut s: &String) {
    let t = s.to_string() + " borrowed";
    println!("I borrowed the string: {}", t);
}

fn borrow_vector(v: &mut Vec<i32>) {
    v.push(1);
    let sum: i32 = v.iter().sum();
    println!("I borrowed the vector and calculated the sum: {}", sum);
}
