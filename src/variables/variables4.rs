// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

/*
    기본적으로 변수는 immutable이며 변수의 값을 변경하고자 할때는 mut 키워드를 사용한다.
*/

fn main() {
    let mut x = 3; // adding keyword 'mut'
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
