struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

struct Point(i32, i32, i32);

struct Custom_struct(i32, String, bool);

fn main() {
    let username = String::from("marceloludovico");
    let email = String::from("marcelo.ludovico@example.com");
    let uri = String::from("https://marceloludovico.com");
    let active = true;

    let u1 = User {
        username,
        email,
        uri,
        active,
    };
    println!("user 1 username is: {}", u1.username);

    let p1 = Point(10, 20, 30);
    println!("p1.0, {}!", p1.0);

    let cs = Custom_struct(42, String::from("Hello"), true);
    println!("Custom_struct values: {}, {}, {}", cs.0, cs.1, cs.2);
}
