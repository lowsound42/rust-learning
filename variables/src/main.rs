use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index.trim().parse().expect("index was not a number!");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
