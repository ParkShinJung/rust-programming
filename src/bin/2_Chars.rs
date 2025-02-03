fn main() {
    // ✨String 타입은 큰 따옴표("")로 감싸고, char 타입은 작은 따옴표('')로 감싼다.
    println!("Hello, world!");  // String
    let first_letter = 'A';     // char
    let space = ' ';            // char
    let other_language = '한';  // char
    let cat_face = '😺';        // char

    // char = 4 bytes

    // casting = simple type change
    let my_number: u16 = 8;
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16;    // u16 + u8 -> u16
    // let third_number = my_number + second_number;    // 타입을 쓰지 않으면 에러
    println!("my_number + second_number = {}", third_number);


    let number = 'a' as u8; // as 키워드를 사용하여 타입을 변환할 수 있다.
    println!("number is {}", number);   // 97
}
