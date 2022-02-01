// Types of struct
// Unnamed but with type
struct Color(i32, i32, i32);

// Named
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

// Unit-like
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(u: &User) {
    println!(
        "User: {}, Mail: {}, Active: {}, Sing in count: {}",
        u.username, u.email, u.active, u.sign_in_count
    );
}

fn main() {
    let user1 = build_user(String::from("someone@mail.com"), String::from("someone"));
    print_user(&user1);

    // Create a new instance without update syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    print_user(&user2);

    // Create a new instance with update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    print_user(&user3);

    // Note that the struct update syntax is like assignment with = because it moves the data,
    // just as we saw in the “Ways Variables and Data Interact: Move” section.
    // In this example, we can no longer use user1 after creating user2 because the String
    // in the username field of user1 was moved into user2. If we had given user2 new String values
    // for both email and username, and thus only used the active and sign_in_count values from user1,
    // then user1 would still be valid after creating user2. The types of active and sign_in_count are
    // types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.

    let _black = Color(0, 0, 0);
    let _subject = AlwaysEqual;
}
