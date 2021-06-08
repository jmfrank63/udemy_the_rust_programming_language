use std::mem;

pub fn main1() {
    // data types
    let bool_variable = "true";
    println!("My bool variable value is {}.", bool_variable);
    let bool_two: bool = false;
    println!("My bool_two variable value is {}.", bool_two);
    let c = 'c';
    println!("My c variable value is {}.", c);
    let number = 42;
    println!("My number variable value is {}.", number);
    let double_number = 1.0;
    println!("My double_number variable value is {}.", double_number);
    let small: u8 = 240;
    println!("My small variable value is {}.", small);
    let signed_small: i8 = -127;
    println!("My signed_small variable value is {}.", signed_small);
    let max_small = std::u8::MAX;
    println!("My max_small variable value is {}.", max_small);
    let min_small: i8 = std::i8::MIN;
    println!("My min_small variable value is {}.", min_small);
    // memory used
    let single_byte = false;
    println!(
        "My single_byte variable memory size is {}",
        mem::size_of_val(&single_byte)
    );
}
