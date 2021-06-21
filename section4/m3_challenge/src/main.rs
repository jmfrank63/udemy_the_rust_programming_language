fn main() {
    let number = 4;
    println!("Can {} be divided by two?", number);
    let division = number % 2;
    match division {
        0 => println!("Yes"),
        _ => println!("No"),
    }
}
