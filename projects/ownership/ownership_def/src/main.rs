fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function ...
                                    // ... and so is no lobger valid here
    let x = 5;                      // x comes into the scope
    makes_copy(x);                  // x would move into the function but i32 is Copy
                                    // so it's okay to still use x afterwards
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // s goes out of scope and `drop' is called to free the memory

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}