// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

/*
    넘겨줘야할 매개변수가 있다면 타입을 꼭 명시해줄 것
*/

fn main() {
    call_me(3);
}

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
