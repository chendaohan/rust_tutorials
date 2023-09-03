use std::{any, mem};

fn main() {
    let int1 = 10;
    let int2: u64 = 20;
    let int3 = 20_000_u64;  // 下划线是方便区分数字和类型标识
    println!("int1 = {:?}, int2 = {:?}, int3 = {:?}", type_name(&int1), type_name(&int2), type_name(&int3));

    let size1 = 10_isize;
    let size2 = 10_usize;
    println!("size1 = {} bit, size2 = {} bit", mem::size_of_val(&size1) * 8, mem::size_of_val(&size2) * 8);

    println!("==================================");

    let overflow = i32::MAX;
    println!("wrapping_add = {}", overflow.wrapping_add(1));
    println!("checked_sub = {:?}", overflow.checked_sub(1));
    println!("checked_add = {:?}", overflow.checked_add(1));
    println!("overflowing_sub = {:?}", overflow.overflowing_sub(1));
    println!("overflowing_add = {:?}", overflow.overflowing_add(1));
    println!("saturating_add = {}", overflow.saturating_add(1));
}

// 返回值的类型
fn type_name<T>(_val: &T) -> &'static str {
    any::type_name::<T>()
}

#[test]
fn float_type() {
    println!("0.3_f64 = {}", 0.1_f64 + 0.2_f64 == 0.3_f64);
    println!("0.3_f32 = {}", 0.1_f32 + 0.2_f32 == 0.3_f32);
    println!("{:x} == {:x}", (0.1_f64 + 0.2_f64).to_bits(), 0.3_f64.to_bits());
    println!("{:x} == {:x}", (0.1_f32 + 0.2_f32).to_bits(), 0.3_f32.to_bits());
    // 如果实在想比较浮点数
    println!("0.3_f64 = {}", (0.1_f64 + 0.2_f64 - 0.3_f64).abs() < f64::EPSILON);
    println!("0.3_f32 = {}", (0.1_f32 + 0.2_f32 - 0.3_f32).abs() < f32::EPSILON);

    println!("=================================");

    println!("sqrt = {}", (-42.0_f32).sqrt());
    println!("NaN = {}", f64::NAN == f64::NAN);
    println!("NaN = {}", f64::NAN.is_nan());
    println!("Infinity = {}", f64::INFINITY == f64::INFINITY);
    println!("Negative Infinity = {}", f64::NEG_INFINITY == f64::NEG_INFINITY);
}

#[test]
fn operations() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("sum = {sum}");
    println!("difference = {difference}");
    println!("product = {product}");
    println!("quotient = {quotient}");
    println!("remainder = {remainder}");

    println!("==========================");

    // 不同类型的整数转换
    let sum2 = 655_i64 + 5_i32 as i64;  // 一定成功
    println!("sum2 = {sum2}");
    let b: i32 = i64::MAX.try_into().expect("类型转换失败");
    let sum3 = 655_i32 + b;
    println!("sum3 = {sum3}");
}

#[test]
fn bitwise_operations() {
    let a = 2_i32;  // 0b00_000_010
    let b = 3_i32;  // 0b00_000_011
    println!("       a = 0b{:032b}", a);
    println!("       b = 0b{:032b}", b);
    println!(" (a & b) = 0b{:032b}", a & b);
    println!(" (a | b) = 0b{:032b}", a | b);
    println!(" (a ^ b) = 0b{:032b}", a ^ b);
    println!("    (!b) = 0b{:032b}", !b);
    println!("(a << b) = 0b{:032b}", a << b);
    println!("(a >> b) = 0b{:032b}", a >> b);
}
