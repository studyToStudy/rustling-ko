// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

/*
    square 함수의 num * num에 ; 세미콜론이 붙어있지 않아야한다.
    ;은 식의 종료를 나타내며 반환값은 () -> empty tuple이기 때문이다.
*/

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
