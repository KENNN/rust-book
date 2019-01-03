fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        active: true,
        sign_in_count: 1,
    };
    print!("{:?}", user1.email);
    println!("{:?}", user1.username);
    println!("{:?}", user1.active);
    println!("{:?}", user1.sign_in_count);


    user1.email = String::from("another@example.com");
    println!("{:?}", user1.email);

    let user2 =  build_user(String::from("someone@example.com2"), String::from("someoneusername345"));
    println!("{:?}", user2.username);

    // using the field init shorthand when variables and fields have the same name
    let user3 = build_user2(String::from("someone@example.com2"), String::from("someoneusername345"));
    println!("{:?}", user3.email);

    // Creating Instances From Other Instances With Struct Update Syntax
    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{:?}", user4.active);

    // Using Tuple Structs without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black.0);
    println!("{:?}", origin.1);

    // Ownership of Struct Data
    /* Error: missing lifetime specifier
    let uwl = UserWithoutLifetime {
        email: "someone@example.com",
        username: "someoneusername123",
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", uwl.email);*/
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

/*struct UserWithoutLifetime {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}*/


fn build_user(email: String, username: String) -> User {
     User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email:String, username:String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}