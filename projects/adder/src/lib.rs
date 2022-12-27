

pub fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

// $cargo test one_hundred







// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }











// pub struct Guess {
//     value: i32,
// }
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }
//         Guess {
//             value
//         }
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     #[should_panic]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }







// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[derive(Debug)]
// pub struct Rectangle {
//     length: u32,
//     width: u32,
// }

// impl Rectangle {
// // Proper working can hold method
//     pub fn can_hold(&self, other: &Rectangle) -> bool {
//         self.length > other.length && self.width > other.width
//     }
// // Bugged can_hold method which will cause tests to fail
// //     pub fn can_hold(&self, other: &Rectangle) -> bool {
// //         self.length < other.length && self.width > other.width
// //     }
// }

// pub fn greeting(name: &str) -> String {
//     String::from("Hello!")
// }

// #[cfg(test)]
// mod tests {
//     // Because the tests modules is an inner module 
//     // we do this so that everything below is available to the 
//     // tests module
//     use super::*;
//     #[test]
//     fn exploration() {
//         assert_eq!(2 + 2, 4);
//     }
//     #[test]
//     fn another() {
//         // Forces a failure
//         panic!("Make this test fail");
//     }
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle { length: 8, width: 7 };
//         let smaller = Rectangle { length: 5, width: 1 };
//         assert!(larger.can_hold(&smaller));
//     }
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle { length: 8, width: 7 };
//         let smaller = Rectangle { length: 5, width: 1 };
//         assert!(!smaller.can_hold(&larger));
//     }
//     // Using assert_eq
//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add(2, 2));
//     }
//     // How to add a custom failure message to your test
//     // fails so it will show you the message
//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, value was `{}`", result
//         );
//     }
// }

// // 250