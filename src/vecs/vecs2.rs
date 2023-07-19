// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

/*
    vec_loop와 vec_map은 동일한 역할을 수행하는 함수이다.
    vec_loop 함수 내부의 반복문은 v를 순회하는데 .iter_mut()를 사용했다.
    .iter_mut()은 변경할 수 있는 레퍼런스를 전달한다. 그래서 element는 &element 형태로 전달되는데 값을 사용하기 위해서는 *을 사용해서
    Deref 형식으로 취해줘야 포인터 너머의 값에 접근 할 수 있게된다.
    따라서 vec_loop은 *element = *element에 2를 곱한 값, 이 되어야 한다.

    vec_map에서는 .map이 사용되었다. ||는 closure이며 ||안에 있는 element는 클로저가 전달하는 인자이다.
    element를 전달하고 있으므로 여기에 2를 곱해주면 된다.
*/

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element = *element * 2
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
