


// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }


// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
//  }

// impl Summary for NewsArticle {}

// fn main(){
//     let mut article = NewsArticle{
//         headline: String::from ("hello"),
//         location: String::from("hello also"),
//         author: String::from("john doe"),
//         content: String::from("boris johnson"),
//     };
//     println!("New article available! {}", article.summarize());
// }


///////////////
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main(){
	let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
	};
    notify(tweet);
}
//////////////////



// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }


// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }





// ////////////////////////////////////////////////////////////////////////////////
// // trait's name is Summary 
// // inside we put the method signatures (methods) that 
// // implement the generic type, and what 
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// let tweet = Tweet {
//     username: String::from("horse_ebooks"),
//     content: String::from("of course, as you probably already know, people"),
//     reply: false,
//     retweet: false,
// };
// println!("1 new tweet: {}", tweet.summarize());

// ////////////////////////////////////////////////////////////////////////////////









// struct Point<T>u {
//     x: Tv,
//     y: Tw,
// }
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);
//     let char_list = vec![’y’, ’m’, ’a’, ’q’];
//     let result = largest_char(&char_list);
//     println!("The largest char is {}", result);
// }

// fn main() {
//     println!("Hello, world!");
// }
