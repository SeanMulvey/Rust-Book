fn main() {

    // String push and pop
    let mut x = String::from("hello");
    println!("{x}");
    // Append "world!" to x
    x.push_str(" world!");
    println!("{x}");
    // Pop the last 7 characters from x ("world!")
    x.pop();
    println!("{x}");
    x.pop();
    println!("{x}");
    x.pop();
    println!("{x}");
    x.pop();
    println!("{x}");
    x.pop();
    println!("{x}");
    x.pop();
    println!("{x}");
    x.pop();
    println!("{x}");

    let y = String::from("Will pass ownership to z");

    let z = y;

    println!("{z}");

}
