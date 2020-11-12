fn main() {
    let mut counter = 0;
    let x = loop {
        counter += 1;
        if counter % 11 == 0 {
            break counter;
        }
    };
    println!("x is now {} and y is {}", x, get_y_value());
    let mut x = x;
    while x != 3 {
        x -= 1;
    }
    println!("x is now {}", x);

    let arr = [x, 5, 4, 3, 2, 1];

    for ele in arr.iter() {
        println!("the ele is {}", ele);
    }
}

fn get_y_value() -> i32 {
    12
}
