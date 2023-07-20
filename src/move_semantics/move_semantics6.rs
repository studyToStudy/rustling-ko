// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

/*
    main의 data 변수는 to_string, 즉 String 타입이고 값의 소유자이다.
    get_char함수의 인자로 data를 넘기게 되면 소유권이 같이 넘어가며
    
    밑 요구사항에 get_char함수는 not take ownership이므로, data변수의 레퍼런스를 보내준다.
    string_uppercase 함수가 ownership을 가져야 하므로
    &String 대신 String을 인자로 받는다.
*/

fn main() {
    let data = "Rust is great!".to_string(); // String => Owner type

    get_char(&data); // Now Owner Here(Moved Value)

    string_uppercase(data); // reference
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}