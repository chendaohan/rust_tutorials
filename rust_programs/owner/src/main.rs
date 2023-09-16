fn main() {
    let str1 = "str1".to_string();
    let str2 = str1;
    // println!("str1 = {str1}");
    println!("str2 = {str2}");

    let str3 = "str3".to_string();
    let str4 = str3.clone();
    println!("str3 = {str3}");
    println!("str4 = {str4}");
}

#[test]
fn copy_trait() {
    let num1 = 5;
    let num2 = num1;
    println!("num1 = {num1}");
    println!("num2 = {num2}");
}

#[test]
fn drop_value() {
    // let str = {
    //     let str = "dream".to_string();
    //     &str
    // };
    // println!("str = {str}");

    let str2 = {
        let str = "dream";
        str
    };
    println!("str2 = {str2}");
}

#[test]
fn move_return() {
    // let str1 = "str1".to_string();
    // takes_ownership(str1);
    // println!("str1 = {str1}");

    let str2 = "str2".to_string();
    let str3 = return_ownership(str2);
    println!("str3 = {str3}");
}

fn takes_ownership(str: String) {
    println!("str = {str}");
}

fn return_ownership(str: String) -> String {
    println!("str = {str}");
    str
}