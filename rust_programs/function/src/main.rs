fn main() {
    print_hello();
    print_world();
    let sum = add(1, 5);
    let result = plus_or_minus(7);
    println!("sum = {sum}");
    println!("result = {result}");

    fn print_hello() {
        println!("hello");
    }
}

fn print_world() -> () {
    println!("world!");
}

fn panic() -> ! {
    panic!("never type example")
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}
