use std::io;

fn main() {
    // println!("Hello, world!");
    // fahrenheit_celsius();
    // celsius_fahreinheit();

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u64 = n
        .trim()
        .parse()
        .expect("Failed to readline");

    let result = fibonacci(n);
    println!("The value of fibonacci series is {result}");
}

// fn fahrenheit_celsius() {
//     println!("Enter the fahrenheit temperature to convert ");
//     let mut f = String::new();
//     io::stdin()
//         .read_line(&mut f)
//         .expect("Failed to read line");
    
//     let f: f64 = f
//         .trim()
//         .parse()
//         .expect("Enter a valid temperature");

//     let c = (f-32.0)*(5.0/9.0);
//     println!("The temperature in celsius is {c}");
// }

// fn celsius_fahreinheit() {
//     println!("Enter the celsius temperature to convert ");
//     let mut c = String::new();
//     io::stdin()
//         .read_line(&mut c)
//         .expect("Failed to read line");
    
//     let c: f64 = c
//         .trim()
//         .parse()
//         .expect("Enter a valid temperature");

//     let f = (c * (9.0 / 5.0)) + 32.0;
//     println!("The temperature in fahrenheit is {:.2}",f);
// }

fn fibonacci(n: u64) -> u64 {
    if n==0 {return 0;}

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b= temp;
    }
    b
}