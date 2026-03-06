fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding...");
    } else {
        println!("Not proceeding...");
    }

    let height = 181;
    if height > 180 {
        println!("You are tall!");
    } else if height > 170 {
        println!("You are average height.");
    } else {
        println!("You are short.");
    }
}
