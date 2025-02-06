// https://dhghomon.github.io/easy_rust/Chapter_14.html
// Strings

fn main() {     // u8
    // String = Sized type
    // str = dynamic type

    let my_name = "sjpark".to_string();     // String type
    let other_name = String::from("sjpark4");    // String type
    // growable + shrinkable
    let mut my_other_name = "sjpark3".to_string();
    my_other_name.push(':');
    println!("{}", my_other_name);
}
