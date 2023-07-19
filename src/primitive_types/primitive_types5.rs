// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

/*
    Destructing은 강력한 문법중 하나로써
    Tuple화 되어있는 값들을 각각의 변수에 할당할 수 있다.
*/

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
