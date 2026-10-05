use std::io;

fn main() {
    println!("Which program would you like to run?");
    println!("(1) Temperature converter\n(2) Nth Fibbonacci number");

    let input = get_one_or_two();

    if input == 1 {
        cf_converter();
    } else if input == 2 {
        fib();
    } else {
        println!("invalid input, quitting....");
    }
}

fn cf_converter() {
    println!("Would you like to convert from (1) C to F or (2) F to C?");
    let resp = get_one_or_two();
    println!("enter your temperature ");
    let num = get_num();
    
    if resp == 1 {
        println!("your temp in F is: {}", num * 9 / 5 + 32);
    } else {
        println!("your temp in F is: {}", (num - 32) * 5 / 9);
    }

}

fn fib() {
    println!("Enter how many numbers into the sequence you'd like to see:");
    let num = get_num();
    let mut first_num = 1;
    let mut second_num = 1;

    for i in (0..num - 2) {
        let inter = first_num + second_num;
        first_num = second_num;
        second_num = inter;
    }

    println!("{second_num}");
}

fn get_one_or_two() -> u8 {
    loop {
        let input: u8 = match u8::try_from(get_num()){
            Ok(num) => num,
            Err(_) => {println!("number does not fit in u8!"); continue},
        };

        if input == 1 || input == 2 {
            break input;
        } else {
            println!("invalid input, try again");
        }
    }
}

fn get_num() -> i32 {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("input error");
        
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {println!("invalid input, try again"); continue},
        };
    }
}