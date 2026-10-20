fn main() {
    // println!("Hello, world!");
    // let mut s = String::from("hello world");
    // s.push_str(", world");
    // println!("{s}");

    // let s1: String = String::from("string 1");

    // //let s2 = s1; not valid
    // let s2 = s1.clone();
    // println!("{s1} and {s2}");

    // in this case, here number is stored in stack and when add(number) is called then copy of value is assigned and so no error
    // when we try to print number after that line 
    // let number = 15;
    // let result = add(number);
    // println!("the number is {number} and the result is {result}");

    // let us take another example with string which stores in heap so 
    // let name = String::from("Sudheer Das");
    // let result = takes_ownership(name);

    // but here if we try to print name then error throws
    // println!("name is {name} and result is {result}");

    // let s1 = gives_ownership();

    // println!("s1 is {s1}");

    // let s2 = takes_and_gives_back(s1);

    // // s1 ownership is given to takes_and_gives_back function then from inside this fn s string ownership is given to s2 
    // println!("s2 is {s2} and s1 can't be printed here due to not having ownership anymore");

    // let s1 = String::from("Sudheer Das");
    // let (s1,len) = calculate_length(s1);

    // println!("s1 is {s1} and result is {len}");
}

// fn takes_ownership(s:String) {
//     println!("inside takes_ownership {s}");
// }

// fn gives_ownership() -> String{
//     let s= String::from("This is from gives ownership function");
//     s
// }

// fn takes_and_gives_back (s:String) -> String{
//     let s= String::from("This is inside takes and gives back ownership function");
//     s
// }

// fn add(x: i32) -> i32 {
//     x+10
// }

// here ownership and length is both returned and this becomes tedious , so rust has references
// fn calculate_length(s:String) -> (String,usize){
//     let result = s.len();
//     (s,result)
// }