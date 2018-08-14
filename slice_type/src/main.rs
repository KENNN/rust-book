fn main() {
	let mut s = String::from("hello world");
	let word = first_word(&s);
	s.clear();
	println!("{:?}", word);

	let s = String::from("hello world");
	let hello = &s[0..5];
	let world = &s[6..11];
	println!("{} {}", hello, world);

	let slice1 = &s[0..2];
	let slice2 = &s[..2];
	println!("{:?} {:?}", slice1, slice2);

	let len = s.len();
	let slice3 = &s[3..len];
	let slice4 = &s[3..];
	println!("{:?} {:?}", slice3, slice4);

	let slice5 = &s[0..len];
	let slice6 = &s[..];
	println!("{:?} {:?}", slice5, slice6);

	let word = first_word2(&s);
	println!("{:?}", word);
    
    // error: cannot make mutable reference after making immutable one 
	// s.clear();

	// String literals are slices, so the're 

	let my_string = String::from("hello, world");
	let word = first_word3(&my_string[..]);
	println!("{:?}", word);
	let my_string_literal = "hello world";
	let word = first_word3(&my_string_literal[..]);
	println!("{:?}", word);
	let word = first_word3(my_string_literal);
	println!("{:?}", word);
}

fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}
	s.len()
}

fn first_word2(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}

fn first_word3(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}