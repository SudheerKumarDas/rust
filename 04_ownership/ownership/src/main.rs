fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello world");
    s.push_str(", world");
    println!("{s}");

    let s1: String = String::from("string 1");

    //let s2 = s1; not valid
    let s2 = s1.clone();
    println!("{s1} and {s2}");
}
