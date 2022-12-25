use std::collections::HashMap;
fn main(){
    // create hashmap
    // hashmaps have less support and no built in macro to make
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Alternatively, zip a vector of tuples together to make a hashmap
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // Values are moved and the hashmap becomes the owner of the value
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // How to access values in a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // How to iterate over each value in a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    Common Collections   147
    // this prints in an ARBITRARY order
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // How to overwrite a value in a hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // How to update a hashmap value base don the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // How to insert a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}


// // String Manipulation
// fn main(){
// //  Declare a string
//     let mut s = String::new();

//     // Declare static data
//     let data = "initial contents";
//     let s = data.to_string();
//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();

//     // Another way to declare the initial contents of a string:
//     let s = String::from("initial contents");

//     // Example valid string declarations:
//     let hello = String::from("ϢϜΘϠϋϡϼδϟ΍");
//     let hello = String::from("Dobrý den");
//     let hello = String::from("Hello");
//     let hello = String::from("ʭˣʬʝʕʹ");
//     let hello = String::from("नमस्ते");
//     let hello = String::from("こんにちは");
//     let hello = String::from("안녕하세요");
//     let hello = String::from("你好");
//     let hello = String::from("Olá");
//     let hello = String::from("Здравствуйте");
//     let hello = String::from("Hola");

//     // update a string 
//     let mut s = String::from("foo");
//     s.push_str("bar");

//     // update a string also, see page 171, has ramifications
//     // if we didnt use push_str and just push, we would only
//     // push a char
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("s2 is {}", s2);

//     // push a single char to a string 
//     let mut s = String::from("lo");
//     s.push('l');

//     // String concatenation
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used


//     // For more complicated string concatenation we can just use the
//     // !format macro
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
//     let s = format!("{}-{}-{}", s1, s2, s3);

//     // Iterate over a string using .chars()
//     for c in "नमस्ते".chars() {
//         println!("{}", c);
//     }

//     // If you just want to return the bytes, you can use the 
//     // bytes method, although printing may of course produce
//     // the incorrect value since some chars are multibyte
//     for b in "नमस्ते".bytes() {
//         println!("{}", b);
//     }
// }


// // Vector usage examples
// fn main() {
//     // Create a new empty vector that holds values of type i32. 
//     let v: Vec<i32> = Vec::new();
//     // The more typical way to create a vector, letting the vec!
//     // macro do it and decide which type to add.
//     let v = vec![1, 2, 3];
//     // To create a vector, then add elements to it, use push, it
//     // has to be mutable of course. 
//     let mut v = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);    
//     ////////////////
//     // reading the elements from a vector
//     let v = vec![1, 2, 3, 4, 5];
//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);
//     // we use the get() function since if there is no third element
//     // get() will just return none, as opposed to failing out like 
//     // it might otherwise. This allows you to handle the case where 
//     // there is no third element.
//     match v.get(2) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     }
//     /////////////////
 
//     // Printing all elements in a vector with a for loop
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{}", i);
//     }
//     // Modifying all elements in a vector with a for loop
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         // Need this to change the value a ref referres to 
//         *i += 50;
//     }

//     // Using an enum in a vector
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// }
