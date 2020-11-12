fn main() {
    let x = {
        let y = 4;
        y
    };
    // comments are usually above the line they're referencing
    different_function(x, get_my_float());
    if x == 3 {
        println!("x equaled 3!");
    } else if x % 3 == 0 {
        println!("x is divisible by 3!");
    } else {
        println!("x did not equal 3 and is not divisible by it :(");
    }
}

fn get_my_float() -> f64 {
    0.1
}

fn different_function(x: i64, y: f64) {
    println!("You passed in {} and {}", x, y);
}
