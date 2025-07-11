fn main() 
{
    // Creating variables with different data types
    let message = "hello, world";
    let x: i32 = 42;  // 32-bit integer
    let pi: f64 = 3.14; // 64-bit float
    let is_rust_fun: bool = true;  // boolean
    let letter_a: char = 'a';  // character

    //--------------------------------------------------
    
    // Creating a function
    fn add(x: i32, y: i32) -> i32  // '->' sign means "return" 
    {
        x + y  // instead of this we can use "return x + y;"
    }
    //--------------------------------------------------

    // Creating if-else statements
    let x = 42;

    if x >= 0
    {
        println!("x is positive");
    } else {
        println!("x is negative");
    }
    //------------------------------

    // Creating while loop
    let mut i = 0;  // mut is mutable 
    while i < 5
    {
        println!("i is {}", i);
        i += 1;  
    }
}
