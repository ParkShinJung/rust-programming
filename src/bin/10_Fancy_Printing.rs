// https://dhghomon.github.io/easy_rust/Chapter_13.html
// More about printing

fn main() {     // u8
    print!("This\nis\na\nfancy\nprint\n\n");
    print!("c:\thisdrive\new_drive");
    print!(r#"c:\thisdrive\new_drive"#);     // raw string

    println!("\nlet me tell you\
    어떤 이야기를\
    봅시다.");

    let my_variable = &9;
    let my_variable2 = 9000;
    let my_variable3 = 14;
    println!("{:p}", my_variable);      // 포인터로 출력
    println!("{:X}", my_variable2);     // 16진수로 출력
    println!("{:b}\n\n\n", my_variable3);     // 바이트로 출력

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right


}
