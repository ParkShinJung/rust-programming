// https://dhghomon.github.io/easy_rust/Chapter_12.html
// The stack, the heap, and pointers

// 속도: stack > heap
// 공간: stack < heap

// ✨ Rust 언어에서 Pointer를 Reference라고 부른다.
fn main() {
    let my_number = 15;
    let single_reference = &my_number;  // reference to my_number  -> single_reference는 my_number의 주소를 가리킨다.
    let double_reference = &single_reference;   // reference to reference to my_number
    let five_reference = &&&&&my_number;

    // reference는 각각 다른 변수의 의미를 가진다. 독립적으로 사용된다.
}
