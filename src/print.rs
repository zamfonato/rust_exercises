use std::fmt;
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);
struct MinMax(i64, i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Min:{}, Max:{})", self.0, self.1)
    }
}
struct Coordinate  {
    x: f64,
    y: f64,
}
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn run(){

    // Print to the console
    println!("Hello from the print.rs file");
    
    // Basic formatting
    println!("{} is from {}","Brad","New York");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}","Brad","New York","code");

     // Named arguments
     println!("{name} is from {city}", name = "Brad",city = "New York");

     // Placeholder traits
     println!("Binary: {:b} Hex: {:x} Octal: {:0}", 10, 10, 10);

     // Placeholder for debug trait
     println!("{:?}", (123,"abc", 'z', true, ()));

     // Basic math
     println!("10 + 10 = {}", 10 + 10);

     // Right-align text with a specified width (5 white spaces)
     println!("{text:>width$}", text="Hello", width=10);

     // Pad numbers with extra zeroes.
    println!("{number:>0width$}", number=1, width=3);

    // Printing structs
    println!("My struct: {:?}", Structure(3));
    println!("Deep struct: {:?}", Deep(Structure(7)));

    let minmax = MinMax(0, 14);
    let coord = Coordinate { x: 14.1, y: 10.5 };

    // Display formated values
    println!("Display: {}", minmax);
    println!("Display: {}", coord);

}