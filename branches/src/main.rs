fn main() {
    let x = 3;
    let y = 5;

    // If conditional syntax 
    if x == y {
        println!("x is equal to y");
    } else {
        println!("x is not equal to y. \n x = {x} \n y = {y}");
    }

    // If else if syntax
    if x == y {
        println!("x is equal to y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is greater than y");
    }

    // Using if in declaration 
    let x = if y < 6 {5} else{0};

    println!("x is {x}");
    let mut count = 0;

    // Loop in declaration
    let z = loop {
        println!("Count is {count}");

        if count > x {
            break count;
        }

        count += 1;
    };
    println!("Loop over.");

    println!("z is {}", z);

    // Loop labels 
    'outer: for i in 0..1000 {
        println!("Outer count is {i}");
        'inner: for y in 0..20 {
            println!("Inner count is {y} ");

            if y > 5 {
                println!("Breaking inner");
                break 'inner;
            }
            if i > 5 {
                println!("Breaking outer");
                break 'outer;
            }
        }

        


    }
    // While loop 
    count = 3;
    let z = while count != 0 {
        println!("{count}");

        count -= 1;
    };
    println!("Blastoff!");

    // Collection iteration
    let x = [1,2,3,4,5];

    for element in x {
        println!("{element}");
    }
    println!("");
    // Iteration based on index
    for element in 0..x.len() {
        println!("{}", x[element]);
    }
    println!("");

    // Reverse iteration
    for element in (0..x.len()).rev(){
        println!("{}", x[element]);
    }
}
