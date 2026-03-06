use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let mut i = 0;
    loop {
        let a: i32 = rng.random();
        println!("a is {} and i is {}\na {}= i", a, i, if a == i { "=" } else { "!" });
        if i == a {
            break;
        } else {
            i += 1;
            continue;
        }
    }
}
