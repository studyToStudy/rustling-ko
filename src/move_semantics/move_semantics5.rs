// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

/*
    mutable reference를 두 번 연속으로 실행할 수 없다.
    즉, 다음과 같은 코드는 에러를 발생시킨다.

    한 번에 두 번 이상의 가변으로 빌릴 수 없다.

fn main() {
    let mut x = 100; // mutalbe x
    ley y = &mut x; // reference mutable x
    let z = &mut x; // Error!
    *y += 100;
    *z += 1000;
}
*/

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}