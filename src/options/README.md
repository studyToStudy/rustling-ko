# Options

옵션 타입은 옵션 값을 나타낸다: 모든 옵션은 Some이고 값을 포함하거나 None이다.
옵션 타입은 Rust에서 매우 일반적이다. 다양한 용도로 사용되기 때문이다.

- Initial Value => 초기 값
- 전체 입력 범위에서 정의되지 않은 함수의 리턴 값(부분 함수)
- 간단한 에러처리를 위한 값
- 구조체의 옵셔널 필드
- 구조체의 수취할 수 있는 필드 (loaned of "taken")
- 옵셔널 함수의 전달인자(arguments)
- Nullable Pointers
- Swapping things out of difficult situations

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
