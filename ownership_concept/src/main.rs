fn main() {
    

    // Fundamentals of Rust Ownership   // Ownership is a set of rules that governs how a Rust program manages memory.
    // Each value in Rust has a variable that is its "owner".// When the owner goes out of scope, the value will be dropped (memory is freed).
    let my_string1 = String::from("iyte");
    let my_string2 = my_string1;

    // println!("my string is: {}", my_string1); // This will cause a compile-time error because my_string1 has been moved to my_string2
    println!("my string1 is: {}", my_string2); // This will work because my_string2 is the owner of the data now

    let num = 5;
    let my_string3 = String::from("ehm");
    let temp = my_string3;  // ownership is moved from my_string3 to temp

    println!("value of num is: {} and value of temp is: {}", num, temp);
}
