fn main() {
    let my_number = number();
    println!("my_number = {}", my_number);

    let tuple = empty_tuple();
    println!("tuple = {:?}", tuple);

}

// () - empty tuple, unit type(void)
fn number() -> i32 {
    8 // ;를 붙이면 반환값으로 인식되지 않는다.
}

fn empty_tuple() {

}

