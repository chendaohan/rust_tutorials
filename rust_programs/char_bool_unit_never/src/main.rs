fn main() {
    let a = 'z';
    let b = 'ℤ';
    let c = '国';
    let d = '😻';
    println!("a = {a}, b = {b}, c = {c}, d = {d}");
    println!("char size = {} bit", std::mem::size_of::<char>() * 8);

    let e = b'a';
    println!("variable b size = {} bit", std::mem::size_of_val(&e) * 8);
}

#[test]
fn bool_type() {
    println!("bool size = {} bit", std::mem::size_of::<bool>() * 8);

    let is_happy = true;
    let is_unhappy = false;
    println!("is happy = {is_happy}, is unhappy = {is_unhappy}");

    println!("1 > 2 = {}", 1 > 2);
    println!("1 == 2 = {}", 1 == 2);
    println!("1 < 2 = {}", 1 < 2);
}

#[test]
fn unit_type() {
    let unit = ();
    println!("unit size = {} bit", std::mem::size_of_val(&unit) * 8);

    fn print_hello() {  // 等于 fn print_hello() -> () {
        println!("Hello!");
    }
    let return_value = print_hello();
    println!("return value = {return_value:?}");

    let a = {let b = 2;};
    println!("a = {a:?}");

    let b = println!("hello");
    println!("b = {b:?}");
}

#[test]
fn never_type() {
    let a = loop {};
    let b = panic!();
    let c = todo!();
    let d = unimplemented!();
    let e = return;
    fn never_function() -> ! {
        panic!()
    }
    let f = never_function();
    let g: u32 = if 1 > 2 { 4 } else { panic!() };
    let h: u32 = match 2 {
        2 => 2,
        _ => panic!()
    };
}
