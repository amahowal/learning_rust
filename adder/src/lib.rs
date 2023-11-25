fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn greeting(name: &str) -> String {
    // This is the correct one
    // format!("Hello {}!", name)
    format!("Hello Jim!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Used for playing with the should_panic annotation
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

// This is an attribute they are metadata about pieces of code
#[cfg(test)]
mod tests {
    use super::*;
    // internal test
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    // it works with Result type
    #[test]
    // this lets us use the question mark operator to fail the test if any variant of Err is
    // returned
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
        // This is how we would test that some variant of Err is returned if we want the inverse
        // case
        // assert!(value.is_err())
    }

    // should_panic tests that the call panics, but isn't very precise since it could panic on
    // anything!!
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(300);
    }

    // expected allows us to specify a SUBSTRING of the expected panic message!!!
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_expected() {
        Guess::new(200);
    }

    // std lib gives us assert equal and assert not equal
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // use format to print custom debugging messages in test
    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // This isn't ass informative as using the optional format argument
        // assert!(result.contains("Carol"));
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // ! means inverted the bool returned by can_hold
        assert!(!smaller.can_hold(&larger));
    }

    // This indicates that this function is a test, it is an "annotation"
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }
}
