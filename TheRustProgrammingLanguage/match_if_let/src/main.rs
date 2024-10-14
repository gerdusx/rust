// Using `if let` to handle enum variants in Rust

// Define an enum to represent a user's account status
enum AccountStatus {
    Active { username: String },
    Inactive,
    Suspended,
}

fn main() {
    let user_status = AccountStatus::Active {
        username: String::from("johndoe"),
    };

    // // Instead of using a full match statement:
    // match user_status {
    //     AccountStatus::Active { username } => println!("Welcome, {}!", username),
    //     _ => println!("Your account is not active."),
    // }

    // Use `if let` for a more concise syntax
    if let AccountStatus::Active { username } = user_status {
        println!("Welcome, {}!", username);
    } else {
        println!("Your account is not active.");
    }
}
