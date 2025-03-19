fn main() {
    // Declare and set constant
    const MINUTES_IN_DAY: u32 = 60 * 24;

    println!("Number of minutes in a day: {}", MINUTES_IN_DAY);

    // Shadowing example
    let x = 5;

    // Shadow x ny using let
    let x = x + 1;

    // Shadow x based on scope
    {
        let x = x * 2;
        println!("Within the inner scope, x is: {x}");
    }

    println!("After exiting the inner scope, x goes back to: {x}");

    // Using shadowing to change type
    let x = "x is now a string";

    println!("{}", x);

    println!("Lets change x to the length of x");

    let x = x.len();

    println!("{}", x);

}
