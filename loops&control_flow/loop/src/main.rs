fn main() {
    let mut x = 0;
    loop {
        println!("x is {}", x);
        x += 1;
        if x > 500 {
            break;
        }
    }
}
