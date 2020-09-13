fn main() {
    another_function(5, 6);
    block_shadow();
}


fn another_function(x: i32, y: i32) {
    print!("The value of x is: {}\n", x);
    print!("The value of y is: {}\n", y);
}

fn block_shadow() {
    let x = 7;

    let y = {
        let x = 3;
        x + 1
    };

    print!("The value of x is: {}\n", x);
    print!("The value of y is: {}\n", y);
}