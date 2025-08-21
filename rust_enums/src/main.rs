fn main() {
    
    let today = Weekdays::Wednesday; // Create an instance of the enum

    let msg1 = Message::Write(String::from("Hello, Rust!")); // Create an instance of the enum with associated data
    let msg2 = Message::Move {x:10, y:20};
    let msg3 = Message::ChangeColor(255, 0, 0);
    let msg4 = Message::Quit;
    
    process_message(msg1); // Call the function to process the message
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);

    let my_pet = Animal::Dog("Black".to_string()); // Create an instance of the Animal enum

    if let Animal::Dog(name) = my_pet  // This is how to use "if let" syntax to match an enum variant
    {
        println!("My pet is a dog named: {}", name);
    }
    else
    {
        println!("My pet is not a dog.");
    }

    let random_msg = Message::Write(String::from("This is a random message."));
    random_msg.call(); // Call the method defined in the Message enum

}

enum Weekdays // This is how to define an enum in Rust
{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum Message  // This is an enum with associated data
{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) 
{
    match msg
    {
        Message::Quit =>
        {
            println!("Quit message received.");
        }
        Message::Move { x,y } =>
        {
            println!("Move to coordinates: ({}, {})", x, y);
        }
        Message::Write(text) =>
        {
            println!("Write message: {}", text);
        }
        Message::ChangeColor(r, g, b) =>
        {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}

enum Animal
{
    Dog(String), // Associated data for Dog
    Cat(String), // Associated data for Cat
    Bird(String), // Associated data for Bird
}

impl Message  // This is how to implement methods for an enum in Rust
{
    fn call(&self)
    {
        match self
        {
            Message::Quit => println!("Exiting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
        }
    }
}