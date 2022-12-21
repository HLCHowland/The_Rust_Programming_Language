// includes the io library (std namespace?)
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //Inclusive on the lower bound but exclusive on the upper, possible off by ones
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);   
    
    // Rust infinite loop syntax 
    loop{
        println!("\nPlease input your guess.");
        let mut guess = String::new();
    
        //1. Take a string and put it into a variable
        //2. &mut makes a mutable reference, this is not like
        // accessing the memory directly.  
        // 3. For result the return types can by an enum of 'Ok' and 'Err'
        // the exepect processes the reult and will deliver the text
        // in case of error. It is enforced error handling.
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        // Rust allows us to shadow the pervious value of guess with a 
        // new one. This is used to convert values from one type to another.
        // This allows us to reuse guess without having to create a new guess 
        // variable. 
        // trim --- this gets rid of beginning and trailing whitespace, and
        //          removes the newline at the end of strings 
        // parse -- this parses a string into a number, we have to tell
        //          it which number type we want to parse to. Since rand
        //          returns 32 bit ints by default, we make this u32.
        // expect - if parse returns an error we have to be able to handle 
        //          that, so we put in an except
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // ....
        // The end piece we just modified and the fact that we made it a match statement
        // here alows us to handle the error in a more fine grain manner. In this
        //  case we just continue if we get an error. 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        // Ordering is another enum, of which the variants for ordering are
        // Less, Greater and Equal 
    
        // Match expressions are made up of arms, this seems to be like
        // a switch case in a sense
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // break inside this location 
                // will cause us to end the loop
                break;
            }
        }
    }
}

// Cargo.lock is created the first time you build. 
// If a new version is released, .3.0 and .4.0
// cargo will get the closest one to .4.0 but not anyone
// in .4.0. Seems like Rust has a standardized update system 
// If you want to update the version of the library to .4.0 
// you will have to manually update the dependencies file
