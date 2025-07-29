fn main() {
    let original_string = String::from("iyte ehm");
    let cloned_string = original_string.clone();  // Cloning the original string using ".clone()"

    println!("Original: {}", original_string);
    println!("Cloned: {}", cloned_string);

    let modified_string = modify_string(&cloned_string);  // Passing the cloned string to the function
    println!("Modified: {}", modified_string);

}

fn modify_string(s:&String) -> String
{
    let mut string1 = s.clone();  // Cloning the string to modify it
    string1.push_str(" modified");
    string1  // return string1;
}
