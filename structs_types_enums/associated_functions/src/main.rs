struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn from_email(email: String) -> Self {
        let username = email.split('@').next().unwrap_or("").to_string();
        Self {
            username,
            email,
            uri: String::new(),
            active: true,
        }
    }

    fn update_uri(&mut self, new_uri: String) {
        self.uri = new_uri;
    }
}

fn main() {
    let mut u1 = User::new(
        String::from("marceloludovico"),
        String::from("marceloludovico@example.com"),
        String::from("https://marceloludovico.com"),
    );
    println!("Hello, {}!", u1.username);
    println!("Account {} status is: {}", u1.username, u1.active);
    u1.deactivate();
    println!("Account {} status is: {}", u1.username, u1.active);

    let mut u2 = User::from_email("marcelo@example.com".to_string());
    println!("Hello, {}!", u2.username);
    u2.update_uri("newuri.com".to_string());
    println!("User {} has URI: {}", u2.username, u2.uri);
}
