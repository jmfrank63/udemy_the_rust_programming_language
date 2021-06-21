fn main() {
    let x = "Apple";
    match x {
        "Apple" => println!("Great brand!"),
        "Linux" => println!("Even better"),
        _ => println!("Some other OS system"),
    }

    let area_code = 206;
    let area = match area_code {
        206 => "Seattle",
        318 => "Louisiana",
        200..=300 => "Washington State",
        _ => "Invalid",
    };
    println!("The area for {} is {}.", area_code, area);
}
