fn main() {
    let height = 181;
    // height = height + 1; // error: cannot assign twice to immutable variable `height`
    let result = if height > 180 {
        "tall"
    } else if height > 170 {
        "average"
    } else {
        "short"
    };

    println!("{}", result);

    let mut mutable_height = 181;
    mutable_height = mutable_height - 11; // works fine
    let mut_result = if mutable_height > 180 {
        "tall"
    } else if mutable_height > 170 {
        "average"
    } else {
        "short"
    };

    println!("{}", mut_result);

    // shadowing variables
    let height = height - 1; // works fine, shadows the previous `height`
    let result = if height > 180 {
        "tall"
    } else if height > 170 {
        "average"
    } else {
        "short"
    };

    println!("{}", result);
}
