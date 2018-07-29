fn main() {

	// variables and mutability
	let x = 5;
    println!("The value of x is: {}", x);

	let x = x + 1;
	let x = x * 2;
    println!("The value of x is: {}", x);


	//data types
    let x = 2.0;
    println!("x: {}", x);

    let y: f32 = 3.0;
    println!("y: {}", y);
    
    let sum = 5 + 30;
    let difference = 95.5 - 4.3;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);

    let t = true;
    let f: bool = false;

    println!("t: {}", t);
    println!("f: {}", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat); 
}
