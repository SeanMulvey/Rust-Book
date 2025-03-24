fn main() {
    println!("Understanding the difference of passing by value vs. passing by reference:");
    let mut x = 1;

    println!("  x is {x}");
    println!("  Result of passing by value: {}", by_val(x));
    println!("  x is still {x}");
    println!("  Call pass by ref");
    by_ref(&mut x);
    println!("  x is now {x}");
}

// Passing by value creates a copy
fn by_val(x: u32) -> u32{
    // Because x is passed by value, the copy is what gets changed
    x + 1

}

// Passing by reference points to the original x
fn by_ref(x: &mut u32) {
    // Because we are pointing to the original x, the original x is changed
    *x += 1;
}
