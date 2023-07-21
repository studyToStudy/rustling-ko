// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

/*
    mod => 새로운 모듈을 만들때 선언한다.
    기본적으로 함수, 타입, 상수, 모듈은 private이다
    pub 키워드를 사용해 public하게 만들어줘야 바깥에서도 볼 수 있다.
*/

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
