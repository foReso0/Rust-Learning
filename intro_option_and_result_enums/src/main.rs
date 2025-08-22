fn main() {
    
    /*enum Option<T> This is how to define the Option enum
    {
        Some(T),
        None,   
    }*/

    let number = -9.0;
    let result_square_root = take_square_root(number);

    let number1 = 8.0;
    let number2 = 0.0;
    let result_division = division_operation(number1, number2);

    match result_square_root
    {
        Some(value) => println!("The square root of {} is {}", number, value),
        None => println!("Cannot compute the square root of a negative number"),
    }

    match result_division
    {
        Ok(value) => println!("{} divided by {} is {}", number1, number2, value),
        Err(error_message) => println!("Error: {}", error_message),
    }

    /*enum Result<T, E>  This is how to define the Result enum
    {
        Ok(T),
        Err(E),
    }*/
}

fn take_square_root(num: f32) -> Option<f32>
{
    if num >= 0.0
    {
        Some(num.sqrt())
    }
    else
    {
        None
    }
}

fn division_operation(num1: f32, num2: f32) -> Result<f32, String>
{
    if num2 == 0.0
    {
        Err("Cannot divide by zero".to_string())
    }
    else
    {
        Ok(num1 / num2)
    }
}

fn get_from_database(key: &str) -> Option<f32>
{
    let database: [(&str, Option<f32>); 2] = [("base", Some(5.0)), ("height", Some(8.0))];

    for (k,v) in database
    {
        if k == key
        {
            return v;
        }
    }
}

fn calculate_triangle_area(base: Option<f32>, height: Option<f32>) -> Result<f32, String>
{
    match (base, height)
    {
        (Some(b), Some(h)) =>
        {
            if b<=0.0 || h<=0.0
            {
                Err("Base and height must be positive numbers".to_string())
            }
            else
            {
                Ok(0.5 * b * h)
            }
        }

        (None, _) => Err("Base is missing".to_string()),
        (_, None) => Err("Height is missing".to_string()),          
    }
}
