fn main() {
    let a = 1;
    {
        let s = "2";
        println!("s = {}", s);
    }

    //s is not in the scope println!("{} {}", a , s);
    println!("a = {}", a);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s = {}", s);

    let s1 = String::from("rust");
    println!("s1 = {}", s1);
    let s2 = s1;
    println!("s2 = {}", s2);
    //println!("s1 = {}", s1); will not work, s1 has been moved

    let s3 = take_ownership(s2);

    //println!("s2 = {}", s2); will not work, s2 has been moved
    println!("s3 = {}", s3);

    println!("s3.lengh = {}", borrow(&s3));

    println!("s3 = {}", s3); //s3 been borrowed, not moved
    

    let mut s4 = String::from("hohoho");
    borrow_mut(&mut s4);
    println!("s4 = {}", s4);
}

fn take_ownership(some_string: String) -> String{
    println!("some_string = {}", some_string);
    some_string
}

fn borrow(s: &String) -> usize {
    s.len()
}

fn borrow_mut(s: &mut String) {
    s.push_str(" ?????")
}
