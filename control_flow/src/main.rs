fn main() {

    let x = 5;
    if x < 4 {
        println!("x < 4");
    } else if x > 4 {
        println!("x > 4");
    } else {
        println!("x == 4");
    }


    let condition = true;
    let v = if condition {
        1
    } else {
        2
    };

    println!("{}", v);


    let mut counter = 0;
    let r = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //does not make sense to me!!!!  why semicolons????
        }
    };

    println!("r {}", r);

    
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");


    let a = [1,2,3];
    for e in a.iter() {
        println!("e {}", e);
    }

     for number in (1..4){
        println!("{}!", number);
    }
}
