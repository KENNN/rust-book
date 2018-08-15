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
}

struct User {
	username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn build_user(email: String, username: String) -> User {
	 User {
    	email: email,
    	username: username,
    	active: true,
    	sign_in_count: 1,
    }
}