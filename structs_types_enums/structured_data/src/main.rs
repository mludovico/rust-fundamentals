#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
    email: String,
    phone_number: Option<String>,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn new(first_name: String, last_name: String, age: Option<u8>, email: String, phone_number: Option<String>) -> Self {
        Person {
            first_name,
            last_name,
            age,
            email,
            phone_number,
        }
    }
}

fn main() {
    Person::new("asd".to_string(), "asd".to_string(), None, "asd".to_string(), None);

    let p1 = Person {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: None,
        email: "john.doe@contoso.com".to_string(),
        phone_number: Some("123-456-7890".to_string()),
    };

    println!("Person: {:#?}", p1);
    println!("Person's full name: {}", p1.full_name());
}
