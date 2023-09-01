fn main() {
    // 声明宏可以使用 ()、[]、{} 包裹参数
    println!("Hello, world!");
    println!["Hello, world!"];
    println!{"Hello, world!"};

    // println!() 的简单使用
    let name = "dream";
    println!("name = {name}");
    println!("name = {}", name);
    let age = 22;
    println!("name = {1}, age = {0}", age, name);
    println!("name = {name2}, age = {age2}", name2 = name, age2 = age);
    let dream = (name, age);
    println!("name = {dream:?}");
    println!("name = {:?}", dream);
}
