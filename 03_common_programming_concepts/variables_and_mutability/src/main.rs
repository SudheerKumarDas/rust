// fn main() {
//     println!("Hello, world!");
//     let mut x=5;
//     println!("The value of x is: {x}");
//     x=6;
//     println!("The new value of x is: {x}");
//     const THREE_HOURS_IN_SECONDS: u32 = 3*60*60;
//     println!("The value of 3hrs in seconds = {THREE_HOURS_IN_SECONDS}");
//     let y=5;
//     let y=y+1;
//     {
//         let y=y*2;
//         println!("The value of y is {y}");
//     }
//     println!("The value of y is {y}");
// }

// use rand::prelude::*;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("Hello, welcome to the guessing game!!");
//     println!("generating a random secret number from 1 to 100");
//     let secret_number = rand::rng().random_range(1..=100);
//     loop {
//         println!("Enter your guess number please ");
//         let mut guess_number = String::new();
//         io::stdin()
//             .read_line(&mut guess_number)
//             .expect("Failed to read read_line");

//         let guess_number: u32 = match guess_number.trim().parse(){
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         match guess_number.cmp(&secret_number) {
//             Ordering::Less => println!("too small"),
//             Ordering::Greater => println!("too big"),
//             Ordering::Equal => {
//                 println!("you guessed the right number");
//                 break;
//             }
//         }
//     }
// }

// use std::io;

// fn main() {
//     let a = [1,2,3,4,5];

//     println!("Enter array index ");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Enter a valid number ");

//     let element = a[index];

//     println!("The value of the element at {index} is {element}");
// }

// fn main() {
//     println!("Hello World !!");
//     another_function();
// }

// fn another_function() {
//     println!("This is another function");
// }

// fn main() {
//     println!("Hello World !!");
//     another_function(5);
// }

// fn another_function(x:i32) {
//     println!("The value of x is {x}");
// }

fn five() -> i32 {
    5
}

fn five_plus_one(x: u32) -> u32 {
    x + 1
}

// fn five_plus_one(x: u32) -> u32 {
//     x + 1;
// }


fn main() {
    let x = five();
    let y: u32 = five_plus_one(5);
    println!("The value of five function is {x}");
    println!("The value of five_plus_one is {y}");
}