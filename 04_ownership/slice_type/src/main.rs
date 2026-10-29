
fn main() {
    println!("Hello, world!");
    let numbers = [10,20,30,40,50];
    println!("{}", numbers[0]);
    println!("{}", numbers[1]);
    println!("{}", numbers[2]);

    // one importtant thing it does not create a new array
    let middle = &numbers[1..4];
    println!("{:?}", middle);

    // the slice type - access data from a collection without owning the collection or copying the data in the collection
    let new_numbers = [1,2,3,4,5,6,7,8,9];
    let slice = &new_numbers[1..4];
    println!("{:?}",slice);
    // slice[2] = 100; can't do this becoz new_numbers is not mutable , this code directly changes the new_numbers collection so new_numbers must be mutalbe

    // let us take another example with mutable array of numbers
    let mut mutable_numbers = [10,20,30,40,50,60,70,80,90,100];
    let sliced_numbers = &mut mutable_numbers[1..5];
    // println!("{:?}",mutable_numbers);    - we cannot access that because sliced_numbers borrowed mutable_numbers
    println!("{:?}", sliced_numbers);
    sliced_numbers[1] = 999;
    println!("{:?}",sliced_numbers);
}
