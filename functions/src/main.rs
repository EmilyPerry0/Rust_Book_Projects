fn main() {
    println!("Hello, world!");

    another_function();
    third_func(5);
    two_vars(11, true);
    let x = five();
    println!("the value of five is: {x}");

}

fn another_function(){
    println!("second func");
}

fn third_func(x: i32){
    println!("var is: {x}");
}

fn two_vars(x: i32, y: bool){
    println!("two vars are {x} and {y}");
}

fn five() -> i32 {
    5
}