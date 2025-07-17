fn main() {
    let _first_bool = true;
    let _second_bool: bool = false;  // both are valid ways to declare a boolean variable

    // Integer types
    let _days_of_month: u8 = 30; // unsigned 8-bit integer, suitable for small numbers
    let _num_of_countries: u16 = 195; // unsigned 16-bit integer, suitable for larger numbers
    let _large_number: u32 = 4_294_967_295; // unsigned 32-bit integer, can hold very large values
    let _huge_number: u64 = 18_446_744_073_709_551_615; // unsigned 64-bit integer, can hold even larger values

    let _unsigned_int_num: u8 = 255; // maximum value for u8
                                     // unsigned integers cannot be negative
                                     
     let _regular_num = 0; // default type is i32, a signed 32-bit integer

     // Floating-point types
     let _pi: f64 = 3.141592653589793; //
     let _pi: f32 = 3.14;

    // Character type
    let _regular_char1: char = 'A'; 
    let _regular_char2 = '1';

    // String type
    let _messsage = "Hello"; // string slice, immutable
    let _regular_string = String::from("Hello"); // owned string, mutable

    // Array type
    let _months_of_the_year: [&str; 12] = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ]; // fixed-size array of string slices

    let _first_month = _months_of_the_year[0]; // accessing the first element of the array
    let _last_month = _months_of_the_year[_months_of_the_year.len() - 1]; // accessing the last element of the array
                                                                               // len is a function which counts the number of elements in the array

    // Slices
    let slice = &_months_of_the_year[0..3]; // slice of the first three months (0-1-2) 3 is excluded
    let _first_element_of_the_slice = slice[0]; // accessing the first element of the slice

    // Tuples holds different types of data inside of it
    let _person = ("Elif", 25);  // tuple containing a string and an integer
    let _name = _person.0; // accessing the first element of the tuple (name)
    let _age = _person.1; // accessing the second element of the tuple (age)     

    // Variables
    let _num = 5; // immutable variable
    /*num = 6;*/ // this will cause an error because _num is immutable                                                        
}
