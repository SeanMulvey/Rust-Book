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

    let mut y = String::from("Will pass ownership to z");

    let mut z = y;

    println!("{z}");

    z.pop();
    z.push_str("y");

    y = z;

    println!("{y}");

    let mut a = String::from("hello");
    a = String::from("ahoy");

    println!("{a}, world!");

    let s = String::from("My String.");

    let b = "My &str.";


    // Pass s.clone() to keep s in scope
    takes_ownership(s.clone());

    let s = take_and_give_ownership(s);

    println!("{s}");

    str_test(b);

    println!("{b}");

    let c = String::from("test");

    let (d, e) = calculate_len(c);

    println!("The size of {d} is {e}");



    

}

fn takes_ownership(s: String){
    println!("{s}");


}

fn str_test(s: &str){
    println!("{s}");
}

fn take_and_give_ownership(s: String) -> String {
    println!("{s}");
    s
}

fn calculate_len(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}
