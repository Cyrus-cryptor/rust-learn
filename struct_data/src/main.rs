fn main() {
    let user = UserType {
        active: false,
        email: "some@some.com",
        username: "some",
        sign_in_count: 1,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct UserType {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
