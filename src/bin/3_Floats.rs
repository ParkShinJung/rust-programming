fn main() {
    // 동일(숫자 사이에 _를 넣어서 가독성을 높일 수 있다.)
    // 숫자 사이에 _는 무시 된다.
    let my_number = 9u8;
    let my_number = 9_u8;

    // 동일
    let other_number = 100000000u64;
    let other_number = 100_000_000u64;

    let third_number = 9.;
    let fourth_number = 9;
    // println!("{}", third_number + fourth_number);   // 타입이 다르기 때문에 에러!
    println!("{}", third_number as i32 + fourth_number);    // as 로 타입을 캐스팅하여 맞춰주면 에러 안남
    println!("{}", third_number + fourth_number as f64);    // as 로 타입을 캐스팅하여 맞춰주면 에러 안남
}
