
fn main() {
    println!("References and Borrowing");

    let s1 = String::from("Sudheer Kumar Das");
    let result = calculate_length(&s1); // here we are only taking reference of s1 not ownership , ownership still belongs to s1
    // and one more thing here we are borrowing the value and so we can't change the original value or alter it because ownership is never transferred 
    // println!("s1 is {s1} and result is {result}");
    
    // not gonna change the value because ownership is never transferred
    change_value(&s1);

    let mut s2 = String::from("Mutable");
    let result2 = change_value_mutable(&mut s2);
    println!("s2 is {s2} and result2 is {result2}");
}

fn calculate_length(s:&String)->usize{
    s.len()
}

fn change_value(s:&String) {
    // won't work
    // s.push_str(", this won;t work");
}

// we can make mutable by making mutable reference

fn change_value_mutable(s : &mut String) -> usize{
    s.push_str(", hello from mutable reference");
    s.len()
}