/*
    Rust에서 String을 생성하는 방법은 두가지가 있다.
    1. .to_string() 사용
    2. String::from을 사용
*/

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> Stirng {
    "blue".to_string(); // or String::from("blue");
}