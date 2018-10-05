fn main() {
    /* dangling refference
    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r: {}", r);
    }*/

    // The Borrow Cheacker
    {
        let x =5;
        let r = & x;
        println!("r: {}", r);
    }
}
