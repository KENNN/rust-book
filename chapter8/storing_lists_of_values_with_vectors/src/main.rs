fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v = vec![1,2,3];
    println!("{:?}", v);

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // Reading Elements of Vectors
    {
        let v = vec![1,2,3,4];
        let third: &i32 = &v[2];
        println!("{:?}", third);
        let third: Option<&i32> = v.get(2);
        println!("{:?}", third);
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; runtime error
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);

    /* error
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(6);
    */

    // Using an Enum to Store Multiple Types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
