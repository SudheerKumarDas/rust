
fn main() {
    println!("References and Borrowing");
    let s1 = String::from("Sudheer Kumar Das");
    let result = calculate_length(&s1);

    println!("s1 is {s1} and result is {result}");
}

fn calculate_length(s:&String)->usize{
    s.len()
}