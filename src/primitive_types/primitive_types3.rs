// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let a = [0; 100]; // Array 생성시 이렇게 작성할 수 있다. 0으로 채워진 100개의 elements

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
