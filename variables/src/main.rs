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

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y , z) = tup;

    println!("x, y, z: {}, {}, {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!("one: {}", one);


}
