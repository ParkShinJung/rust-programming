fn main() {
    // + plus sign
    // - minus sign
    // i8, i16, i32, i64, i128, and isize. // 정수 포함 모든 숫자 타입
    // u8, u16, u32, u64, u128, and usize. // 정수
    // computer architecture 에 따른 것
    // isize -> 32비트 -> i32
    // usize -> 64비트 -> i64

    // let my_number = 100;    // i8인지 u8인지 명시 하지 않으면 기본 적으로 i32로 인식!!
    let my_number = 100;    // 255
    // let my_number: i8 = 100;    // 라면 에러

    let my_other_number = 50;   // i32

    let third_number = my_number + my_other_number;
    println!("my_number + my_other_number = {}", third_number);
    // type inference
}
