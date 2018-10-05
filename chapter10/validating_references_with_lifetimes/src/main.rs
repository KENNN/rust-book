fn main() {
    /* dangling refference
    {
        let r;                  // ---------+--'a
        {                       //          |
            let x = 5;          // --+'b    |
            r = &x;             //   |      |
        }                       // --+      |
        println!("r: {}", r);   //          |
    }*/                         // ---------+

    // The Borrow Cheacker
    {
        let x = 5;              // ---------+--'b
                                //          |
        let r = & x;            // --+--'a  |
                                //   |      |
        println!("r: {}", r);   //   |      |
                                // --+      |
    }                           // ---------+
}
