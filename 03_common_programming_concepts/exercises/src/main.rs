use std::io;

fn main() {
    println!("Hello, world!");
    fahrenheit_celsius();
    celsius_fahreinheit();
}

fn fahrenheit_celsius() {
    println!("Enter the fahrenheit temperature to convert ");
    let mut f = String::new();
    io::stdin()
        .read_line(&mut f)
        .expect("Failed to read line");
    
    let f: f64 = f
        .trim()
        .parse()
        .expect("Enter a valid temperature");

    let c = (f-32.0)*(5.0/9.0);
    println!("The temperature in celsius is {c}");
}

fn celsius_fahreinheit() {
    println!("Enter the celsius temperature to convert ");
    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Failed to read line");
    
    let c: f64 = c
        .trim()
        .parse()
        .expect("Enter a valid temperature");

    let f = (c * (9.0 / 5.0)) + 32.0;
    println!("The temperature in fahrenheit is {:.2}",f);
}