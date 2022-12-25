

// fn main() {
// // You can make a mutable string like so
//     let mut s = String::from("hello");
//     s.push_str(", world!"); // push_str() appends a literal to a String
//     println!("{}", s); // this will print `hello, world!`
// }


fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s’s value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to
                                    // still use x afterward
} // Here, x goes out of scope, then s. But because s’s value was moved, 
  // nothing special happens.
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

// mutable pass by reference
fn main() {
  let mut s = String::from("hello");
  change(&mut s);
}
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

// Cool slices example
fn main() {
  let my_string = String::from("hello world");
  // first_word works on slices of `String`s
  let word = first_word(&my_string[..]);
  let my_string_literal = "hello world";
  // first_word works on slices of string literals
  let word = first_word(&my_string_literal[..]);
  // Because string literals *are* string slices already,
  // this works too, without the slice syntax!
  let word = first_word(my_string_literal);
}


let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("anotherusername567"),
  active: user1.active,
  sign_in_count: user1.sign_in_count,
};


fn build_user(email: String, username: String) -> User {
  User {
      email,
      username,
      active: true,
      sign_in_count: 1,
  }
}



let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("anotherusername567"),
  ..user1
};


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
  fn area(&self) -> u32 {
      self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
  }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}





impl Message {
  fn call(&self) {
      // method body would be defined here
  }
}
let m = Message::Write(String::from("hello"));
m.call();

enum Option<T> {
  Some(T),
  None,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}



fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}




fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => {
          println!("Lucky penny!");
          1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}




let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}











#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
          println!("State quarter from {:?}!", state);
          25
      },
  }
}







