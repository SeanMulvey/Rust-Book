fn main() {
    println!("Hello, world!");
    println!("Calling another function:");
    another_function();
    println!("Passing in a single parameter:");
    param_function(5);
    println!("Passing in multiple parameters:");
    multi_param_function(3,'f');
    println!("Expression function:");
    expression_function();
    println!("Implicit return function (function with just 5):");
    println!("  implicit_return() returns: {}", implicit_return());
    println!("Function that implicitly returns the paramter(10) * 2");
    println!("double() returns: {}", double(10));
}

fn another_function(){
    println!("  Another function");
}

fn param_function(x: i32){
    println!("  x is {x}");
}

fn multi_param_function(x: u32, c: char){

        println!("  The int param is {x}");
        println!("  The char param is {c}");
}

fn expression_function(){
    let x = {
        let y = 5;
        (y + 5) * 2
    };
    println!("  The value of x is: {x}");
}

fn implicit_return() -> u32{
    5
}

fn double(x: u32) -> u32 {
    x * 2
}
