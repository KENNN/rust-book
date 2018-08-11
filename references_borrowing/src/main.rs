fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // error: cannot borrow as mutable
    // let s = String::from("hello");
    // change(&s);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {}", s);


	/* error: we can have only one mutable reference
    let mut s = String::from("hello");
    let r1 = &mut s;
   	let r2 = &mut s;
   	println!("r1: {}, r2: {}", r1, r2);
   	*/

   	let mut s = String::from("hello");
   	{
   		let r1 = &mut s;
   		println!("r1: {}", r1);
   	}
   	let r2 = &mut s;
   	println!("r2: {}", r2);

   	/* error: but multiple immutable references thireselves are ok
   	let mut s = String::from("hello");
   	let r1 = &s;
   	let r2 = &s;
   	let r3 = &mut s;
   	*/

	/* error: dangling refference 
   	let reference_to_nothing = dangle();
	*/

	let reference_no_dangling = no_dangle();
	println!("no dangling: {}", reference_no_dangling);
}

fn calculate_length(s: &String) -> usize {
	s.len()
}

/*
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

fn change(some_string: &mut String) {
	some_string.push_str(", world");
}

/* 
fn dangle() -> &String {
	let s = String::from("hello");
	&s
}
*/

fn no_dangle() -> String {
	let s = String::from("hello");
	s
}



