fn main() 
{


let employee = Employee // Creating an instance of the Employee struct with sample data.
{ 
name: String::from("Elif Yıldırım"), 
position: String::from("R&D Specialist"),
age:25, 
salary:61000
};

println!("Employee Name: {}", employee.name);
println!("Employee Title: {}", employee.position);
println!("Employee Age: {}", employee.age);
println!("Employee Salary: {}", employee.salary);
//------------------------------------------------------------------------

// Creating another instance of the Employee struct using 'mutable' property
let mut employee2 = Employee
{
name: String::from("Ahmet Demir"),
position: String::from("Software Engineer"),
age: 30,
salary: 80000,
};
employee2.salary += 5000; // We are able to update the salary of employee2 because it is mutable.
println!("\nUpdated Employee Salary: {}", employee2.salary);
println!("\nEmployee 2 Name: {}, Employee 2 Title: {}, Employee 2 Age: {}, Employee 2 Salary: {}", employee2.name,
employee2.position, employee2.age, employee2.salary);
//------------------------------------------------------------------------

// Creating an array of Employee structs using with function
let employee_data = get_employee_data(employee);
println!("\nEmployee Data: {:?}", employee_data);
//------------------------------------------------------------------------

// Creating a new employee using the create_employee function
// This function takes parameters and returns an instance of Employee struct.
let new_employee = create_employee(String::from("Zeynep Kaya"),
String::from("Data Analyst"),
28,
70000);
println!("\nNew Employee Data: {:?}", get_employee_data(new_employee));  //{:?} is used to print the array in a debug format

let my_employee = create_employee("Ali Özçelik".to_string(),"Project Manager".to_string(),35,90000);
println!("\nMy Employee is {:?}", my_employee); 
// This line prints the employee data in a debug format using the `Debug` trait.}
//--------------------------------------------------------------------------------------

// Creating a tuple struct named `Tuple_Employee` and initializing it with data
// This is a tuple struct, which is a simpler way to define a struct without named fields
// It is useful for cases where you want to group related data without needing named fields.
// Tuple structs are defined with parentheses instead of curly braces.
let tuple_employee = Tuple_Employee("Ali Özçelik".to_string(), "Project Manager".to_string(), 35, 90000);
println!("\nTuple Employee -> name:{},position:{}, age:{},salary:{} ", tuple_employee.0,
tuple_employee.1,tuple_employee.2,tuple_employee.3); // This prints the tuple struct using the
//---------------------------------------------------------------------------------------

// Creating a unit struct named `Unit_Employee`
// Unit structs are used when you want to define a type without any data.
let unit_employee = Unit_Employee; // No data is associated with this struct.
//---------------------------------------------------------------------------------------


// Creating a rectangle instance and calculating its area using the `area` method
let reactangle = Reactangle { width: 5.0, height: 10.0 };
let area = reactangle.area(); // Calling the area method to calculate the area of the rectangle
println!("\nRectangle Area: {}", area); // Printing the area of the rectangle
//---------------------------------------------------------------------------------------

}
#[derive(Debug)] // This allows us to print the struct using {:?} in println!

struct Employee  // This is a simple defining of a struct named `Employee`.
{
    name: String,
    position: String,
    age: u32,
    salary: u64,
}

fn get_employee_data(employee: Employee) -> [String; 4]
{
    let employee_name = employee.name;
    let employee_position = employee.position;
    let employee_age = employee.age.to_string();  // to convert age to string
    let employee_salary = employee.salary.to_string();
    let employee_data = [employee_name, employee_position, employee_age, employee_salary];
    employee_data
}

// Function to create an Employee instance with given parameters and it returns a struct
fn create_employee(name: String, position: String, age: u32, salary: u64) -> Employee
{
    let employee = Employee
    {
        name,
        position,
        age,
        salary,
    };
    employee // return employee;
//--------------------------------------------------------------------------------------
}

struct Tuple_Employee(String, String, u32, u64); // This is a tuple struct named `tuple_employee` with four fields.

struct Unit_Employee; // This is a unit struct named `Unit_Employee` with no fields.

struct Reactangle // Defining a struct named `Reactangle` with two fields: width and height.
{
    width: f32,
    height: f32,
}

impl Reactangle // Implementing methods for the Reactangle struct
{
    fn area(&self) -> f32
    {
        self.width * self.height
    }
}

