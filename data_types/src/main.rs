use std::io;
fn main() {
    // Different scalar types
    println!("A scalar type represents a single value");
    println!("Rust has 4 different scalar types");

    
    println!("Integers (128-bit, 64-bit, 32-bit, 16-bit, 8-bit, arch(The architecture of the computer IE 64-bit 32-bit)):");
    let x: u32 = 20000;
    println!("  Unsigned 32 bit integers => u32: {x}");
    let x: i32 = -20000;
    println!("  Signed 32 bit integers => i32: {x}");
    let x: u8 = 8;
    println!("  Unsigned 8 bit integers (0-15) => u8 {x}");

    println!("  etc...");

    println!("Booleans (true/false): ");
    let x: bool = false;
    println!("  x is {x}");

    println!("Characters: ");
    let x: char = 'a';
    println!("  x is {x}");

    println!("Floating-Point (non whole numbers): ");
    let x: f64 = 3.14159265359;
    println!("  x as pi: {x}");


    println!("\n\n\n");
    println!("Rust also has 2 primitive compound types");
    println!("Tuples: ");
    let me: (&str, &str, u16) = ("Sean", "Mulvey", 26);
    println!("  Make a tuple with: \n   let me: (&str, &str, u16) = (Sean, Mulvey, 26) and access each element");

    println!("          First Name: {}", me.0);
    println!("          Last Name: {}", me.1);
    println!("          Age: {}", me.2);


    println!("Arrays(The righteous man's datastructure(seriously, 95% of the time an array is the best structure to use in practice)): ");
    let mut x = [1,2,3,4,5];

    println!("Can be iterated through like every other language.");

    for i in 0..x.len() {
        println!("Element {i} is holding {}",x[i]);
    }

    let mut index = String::new();
    let mut numi = 0;
    // Loop until valid input
    loop {
        println!("Enter the index you want to get.");
    

        io::stdin()
            .read_line(&mut index)
            .expect("Error reading line!");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

        };
        if index < 0 {
            println!("Out of bounds. Enter a valid index!");
        }
        else if index >= x.len().try_into().unwrap() {
            println!("Out of bounds. Enter a valid index!");
        }
        else {
            let element = x[index];
            println!("The value of the element at index {index} is: {element}");
            break;
        }

    }

    
    
    

}
