fn main() {
    // If we want to call our functions, we can do so here.

    let result = adding(5, 10);  // calling the adding function with arguments
    println!("The result of adding is: {}", result);  // printing the value of result

    no_parameters();  // calling the function with no parameters

    let result2 = no_parameters2();  // calling the function that returns an integer
    println!("The result of no_parameters2 function is: {}", result2);
    //---------------------------------------------------------------------------------

    // Control flow example
    let regular_day = "Saturday";

    if regular_day == "Saturday"
    {
        println!("Weekend started!");
    }
    else if regular_day == "Sunday"
    {
        println!("Weekend is ending!");
    }
    else
    {
        println!("It's a regular day.");
    }
    //---------------------------------------------------------------------------------

    // Loop example 1 - while loop
    let mut mutable_number = 0;  

    while mutable_number < 5
    {
        println!("Current mutable number is : {}", mutable_number);
        mutable_number += 1;  // incrementing the mutable number
        println!("Mutable number incremented to: {}", mutable_number);
    }
    //---------------------------------------------------------------------------------

    // Loop example 2 - for loop
    let numbers = [1, 2, 3, 4, 5];  // an array of numbers
    for number in numbers {  // iterating over the array
        println!("Current number is: {}", number);  // printing each number
    }
    for number in 1..=5 {  // Using like this is the same as above
        println!("Current number in range is: {}", number);  // printing each number in the range
    }
    //---------------------------------------------------------------------------------

    // Loop example 3 - loop with break
    let mut counter = 0;
    loop  // We can use loop to create an infinite loop
    {
        counter += 1; 

        if counter == 3 {  // checking if counter equals 3
            println!("Breaking the loop at counter: {}", counter);
            break;  
        }
    }
    //---------------------------------------------------------------------------------

    // Match statement example
    let number = 3;
    match number {  // Since the number is 3, it will match the case for 3
        1 => println!("The number is one."),
        2 => println!("The number is two."),
        3 => println!("The number is three."),
        _ => println!("The number is something else."),  // the underscore _ acts as a catch-all
    }

    let result = match number {  // Using match to assign a value based on the number
        1 => 10,
        2 => 20,
        3 => 30,
        _ => 0,  // default case
    };
    println!("The result from match is: {}", result);  // printing the result from match
    //---------------------------------------------------------------------------------
}

// A simple function that adds two numbers
fn adding(num1: i32, num2: i32) -> i32 {  // num1 and num2 are parameters
    let total = num1 + num2;  // total is a local variable
    return total;  // return statement so the function returns the value of total
}
//---------------------------------------------------------------------------------
fn no_parameters() { // this function does not return anything
        println!("This function has no parameters.");  
    } 
//---------------------------------------------------------------------------------
fn no_parameters2() -> i32 {
        println!("This function has no parameters but returns an integer.");
        42  // this is the same thing as return 42;
}
