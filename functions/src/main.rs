fn main() {
    println!("Hello, world!");
    another_function1();
    another_function2(3);
    another_function3(5, 6);

    let x = 5;
    let y  = {
    	let x = 3;
    	x + 1 
    };
    println!("y: {}", y);

    let x = five();
    println!("x: {}", x);
}

fn another_function1() {
	println!("Another function.");	
}

fn another_function2(x: i32) {
	println!("x: {}", x);	
}

fn another_function3(x: i32, y: i32) {
	println!("x: {}", x);	
	println!("y: {}", y);	
}

fn five() -> i32 {
	5
}