fn main() {
    let mut length = 0;     // 定义了一个可变的变量
    println!("length = {length}");
    length = 3;
    println!("length = {length}");
}

#[test]
fn underline() {
    let _ = 0;           // 使用 _ 命名的变量没有进行绑定
    let _x = 5;     // 开头使用下划线命名的未使用变量
    let y = 10;     // 开头没有使用下划线命名的未使用变量
}

#[test]
fn deconstruction() {
    let dream = ("dream", 22);    // 数组和元组，如果其中的每个元素都实现了 Copy Trait 那么这个数组或元组整体就实现了 Copy Trait
    println!("dream = {dream:?}");
    let (name, age) = dream;
    println!("name = {name}, age = {age}");

    // 品牌， 重量， 主摄数量， 核心数， 刷新频率
    let phone = ("Redme", 180.3, 3, 8_u32, 120_u32);
    let (brand, .., core, frequency) = phone;
    println!("brand = {brand}, core = {core}, frequency = {frequency}");
    let (_, weight, _, _, frequency) = phone;
    println!("weight = {weight}, frequency = {frequency}");

    // 变量遮蔽
    let dream = Some(dream);
    let Some((name, age)) = dream else {
        panic!("dream = None");     // 还可以使用 return;
    };
    println!("name = {name}, age = {age}");
}
