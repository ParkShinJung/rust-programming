fn main() {
    // give_number 함수 호출
    let my_number = give_number(8, 9);
    println!("my_number = {}", my_number);

    // print_number 함수 호출
    print_number(8, 8);

    // multiplied_give_number 함수 호출
    let my_numbers = multiplied_give_number(9, 1);
    println!("my_numbers = {}", my_numbers);

}

fn give_number(one: i32, two: i32) -> i32 {
    one * two
}

fn print_number(one: i32, two: i32) {
    let multiplied = one * two;
    println!("multiplied = {}", multiplied);
}

fn multiplied_give_number(one: i16, two: i16) -> i16 {
    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };
    multiplied_by_ten
}
