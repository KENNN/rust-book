#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // Checking Results with the assert! Macro
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

     #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn larger_can_hold_smaller_with_bug() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold_with_bug(&smaller));
    }

    // Testing Equality with the asseet_eq! and assert_ne! Macros
    #[test]
    fn if_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn if_not_add_three() {
        assert_ne!(5, add_two(2));
    }

    // Adding Custom Failure Messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
                "Greeting did not contain name, value was `{}`", result
        );
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    pub fn can_hold_with_bug(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width < other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
