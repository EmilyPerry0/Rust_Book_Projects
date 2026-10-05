fn main() {
    let num = 6;

    if num < 5 {
        println!("condition met!");
    } else if num < 8 {
        println!("second condition met");
    } else {
        println!("not met");
    }

    let x  = counter_twenty();
    println!("the value was: {x}");
}

fn counter_twenty() -> i32 {
    let mut count = 0;

    loop {
        if count >= 10 {
            break count * 2;
        } else {
            count += 1;
        }
    }
}
