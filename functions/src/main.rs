fn main() {
    println!("Hello, world!");

    another_function(123);

    add_num(2,3);

    let x = {
        1
    };
    println!("{}", x);
    println!("3 * 3 = {}", power2(3));
}

fn another_function(x : i32) {
    println!("you passed {}", x);
}

fn add_num(x: i32, y:i32) {
    println!("{} + {} + {}", x, y, x + y);
}

fn power2(x: i32) -> i32 {
    x * x //or return x * x without semicolons
}