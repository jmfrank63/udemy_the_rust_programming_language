struct Student {
    age: u8,
    mark: u8,
}
#[derive(Debug)]
enum Languages {
    Rust,
    C,
    Swift,
    Java,
    Scala,
}
fn main() {
    let student:Student = Student { age: 23, mark: 17};
    println!("Student information: age: {}, mark: {}", student.age, student.mark);

    let Student {age: my_age, mark:my_mark} = student;
    println!("Information gathered: {} - {}", my_age, my_mark);

    let first_language = Languages::Java;
    println!("{:?}", first_language);

    let second_language = Languages::Rust;
    println!("{:?}", second_language);

    let number = Some(7);
    let letter:  Option<i32> = Some(40);

    if let Some(i) = number {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number.");
    }
    if let Some(l) = letter {
        println!("Matched: {:?}", l);
    } else {
        println!("Didn't match a number");
    }
}
