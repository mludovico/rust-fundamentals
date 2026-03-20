fn get_item(index: usize) {
    let index = 3;
    let v = vec![1, 2, 3, 4, 5];
    let value = v.get(index);
    println!("The value at index {} is: {:?}", index, value);
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    let third_value = vec[2];
    println!("The third value is: {}", third_value);

    match vec.first() {
        None => {
            println!("The vector is empty.");
        }
        Some(first_value) => {
            println!("The first value is: {}", first_value);
        }
    }
}
