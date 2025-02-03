// macro = functuin that writes code
fn main() {
    println!("Hello, world!");

    let my_name = "sjpark";
    let my_age = 28;

    println!("my name is {} and my age is {}", my_name, my_age);
    println!("my name is {} and my age is {}", my_name, give_age());    // 함수 호출
}

fn give_age() -> i32 {
    28
}
