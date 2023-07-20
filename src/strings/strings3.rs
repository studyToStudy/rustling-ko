// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.


/*
    trim() -> 공백을 제거할때 사용한다.
    input은 &str이지만 반환은 String에 유의
*/

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
}

/*
    input type -> &str
    return type -> String
    우선 a를 to_string() 메소드를 사용해서 String으로 바꿔준다. 대신 mut 키워드를 사용해서 immu -> mut 할 수 있게
    push_str()을 사용해서 텍스트 추가
*/
fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut a = input.to_string();
    a.push_str(" world!");
    a
}

/*
    replace(a, b) => a를 b로 Replace한다.
*/
fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut a = input.replace("cars", "balloons");
    a.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
