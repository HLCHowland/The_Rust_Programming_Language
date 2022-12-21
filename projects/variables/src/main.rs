
// // Shadowing example, since X was not declared as mutable 
// // it's like creating a new variable multiple times
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is: {}", x);

//     // here we are able to change a varaibles type through
//     // shadowing as it is like we are creating a new variable
//     let spaces = "   ";
//     let spaces = spaces.len();

//     // floating point values are f32 and f64

// }

fn main() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
}


// also has boolean type
// rust's char type is four bytes to represent a unicode scalar

// Compound Types
// the two compound types are tuples and arrays

fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}