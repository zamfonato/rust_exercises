// Variables are immutable by default
// Block-scoped language

#[allow(dead_code)]
pub fn run() {
    let name: &str = "Pedro";
    let mut age = 31;
    println!("My name is {} and I am {}", name, age);
    // reassign mutable var 
    age += 1;
    println!("My name is {} and I am {}", name, age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Pedro", 31);
    println!("{} is {}", my_name, my_age);

    // Shadowing
    let x = 1;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    // Shadowing with different types
    let x: i32 = 1;
    let x: &str = "x";
    println!("The value of x is: {}", x);

}