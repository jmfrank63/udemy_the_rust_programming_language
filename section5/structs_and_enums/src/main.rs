// #![feature(box_syntax)]
// #![feature(layout_for_ptr)]
use std::mem;
//use arr::Array;
use std::convert::TryInto;

struct Student {
    age: u8,
    mark: u8,
}
#[derive(Debug)]
enum Languages {
    Rust,
    Java,
}

/// A macro similar to `vec![$elem; $size]` which returns a boxed array.
///
/// ```rustc
///     let _: Box<[u8; 1024]> = box_array![0; 1024];
/// ```
macro_rules! box_array {
    ($val:expr ; $len:expr) => {{
        // Use a generic function so that the pointer cast remains type-safe
        fn vec_to_boxed_array<T>(vec: Vec<T>) -> Box<[T; $len]> {
            let boxed_slice = vec.into_boxed_slice();

            let ptr = ::std::boxed::Box::into_raw(boxed_slice) as *mut [T; $len];

            unsafe { Box::from_raw(ptr) }
        }

        vec_to_boxed_array(vec![$val; $len])
    }};
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


    let array_one = [1, 2, 3, 4, 5, 6];
    println!("{}", mem::size_of_val(&array_one));
    const X: usize = 10_000_000;
    let failing_heap_array = [1; X];
    // let huge_array= Box::into_raw(Box::new([1; X]));
    let ha1 = box_array![1; X];
    // let ha2= box [1; X];
    // let ha3 = Array::new_from_template(X, &1);
    println!("{}, {}", mem::size_of_val(&ha1), mem::size_of_val(&*ha1));
    // println!("{}, {}", mem::size_of_val(&ha2), mem::size_of_val(&*ha2));
    // println!("{}", mem::size_of_val(&ha3));
    // let big_2d_array: Array<[i32; X]> = Array::zero(X);
    // println!("{}", mem::size_of_val(&big_2d_array));
    let ha4 = vec![1, X].into_boxed_slice();
    println!("{}, {}", mem::size_of_val(&ha4), mem::size_of_val(&*ha4));

    fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
        v.try_into()
            .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
    }
}
