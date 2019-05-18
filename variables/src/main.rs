fn main() {
    //mutable
    let mut x = 1;
    println!("x {}", x);
    x = 2;
    println!("x {}", x);
    //shadow
    let y = 1;
    println!("y {}", y);
    let y = y + 1;
    println!("y {}", y);
    let y = 3;
    println!("y {}", y);
    //y = 4 is not working, not mutable;
    let y = "hello"; //still ok to be shadowing
    println!("y {}", y);

    //
    let tup: (bool, i32, f32, char) = (true, 130, 130.0, 'a');
    let (a,b,c,d) = tup;
    println!("a {} b {} c {} d {}", a,b,c,d);
    println!("tuple {} {} {} {}", tup.0, tup.1, tup.2, tup.3);

    let ai:[i32;3] = [1,2,3];
    println!("ai {} {} {}", ai[0],ai[1],ai[2]);
}
