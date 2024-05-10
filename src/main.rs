use std::io::Split;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, Templates!");

    let mut user1 = User {
        active: false,
        email: String::from("example@email.com"),
        sign_in_count: 69,
        username: String::from("signInMcSignALot"),
    };

    user1.email = String::from("another@email.com");

    let user2 = create_user(&String::from("cranky@armadillo.com"));

    println!("User1 is {}, {}, {}", user1.username, user1.email, user1.active);
    println!("User2 is {}, {}, {}", user2.username, user2.email, user2.active);
}

fn create_user(email: &String) -> User {
    let _username = word_before_AT(&email);
    User {
        active: true,
        sign_in_count: 1,
        email: String::from(email),
        // username: _username,
        username: String::from(_username),
    }
}

fn word_before_AT(s: &String) -> &str {
    let bytes: Vec<&str> = s.split('@').collect();
    let mut first_word="";
    for w in bytes {
        first_word = w;
        break
    }
    &first_word
}
