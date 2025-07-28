fn main() {
    // Immutable reference example
    let string1 = String::from("Hello, ");
    let reference1 = &string1; // Create a immutable reference to string1
    println!("Immutable reference: {}", reference1); // Immutable reference can be used only for reading the value
    read_string(&string1); // Pass the immutable reference to a function
    //--------------------------------------------------------------------------------------------------------------

    // Mutable reference example
    let mut string2 = String::from("Hi, ");
    modify_string(&mut string2);  // This is the same as modify_string(&mut string2);
    println!("Modified string: {}", string2); // Print the modified string
    //--------------------------------------------------------------------------------------------------------------

    // Multiple immutable references example
    let string3 = String::from("Hello, World!");
    let reference2 = &string3; // Create another immutable reference
    let reference3 = &string3; // Create yet another immutable reference
    println!("Multiple immutable references: {}, {}", reference2, reference3); // All references
    // we can use multiple immutable references at the same time but we cannot use multiple mutable references at the same time
    //---------------------------------------------------------------------------------------------------------------
    //Here's an example of how a dangling reference can occur in Rust:
    let s = String::from("Hello");
    let reference_to_s = return_reference(&s);
    println!("{}", reference_to_s);
    //---------------------------------------------------------------------------------------------------------------
}

fn read_string(s: &String) 
{
    println!("Reading string: {}", s);
}

fn modify_string(s: &mut String)
{
    s.push_str("Guys!"); // Modify the string by appending to it 
}

fn return_reference(some_string: &String) -> &String {
    some_string
} // some_string goes out of scope, but reference_to_s still points to the memory location