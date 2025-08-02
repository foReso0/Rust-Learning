fn main() 
{
    let string1 = "Hello, ";
    let string2 = "world!";
    let concatenated_string = concatenate_strings(string1, string2);
    println!("{}", concatenated_string);

}

fn concatenate_strings(string_slice1: &str, string_slice2: &str) -> String 
{
    let mut result = String::new();
    result.push_str(string_slice1);
    result.push_str(string_slice2);
    result
}
