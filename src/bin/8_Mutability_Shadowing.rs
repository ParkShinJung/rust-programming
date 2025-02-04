// mutability
// shadowing = 같은 이름을 다시 쓰는 것

// immutable by default
// mut
fn main() {
    // mutability-1
    // 하단의 방법으로는 변수를 변경할 수 없다.
    // let my_number = 10;
    // my_number = 9;

    // mutability-2
    // mut 키워드를 사용하면 변수를 변경할 수 있다.
    let mut my_number = 10;
    my_number = 9;
    println!("my_number = {}", my_number);


    // shadowing-1
    let my_variable = 10;
    {
        let my_variable = 20;
        println!("my_variable = {}", my_variable);
    }

    // shadowing-2
    let x = 9;
    let x = double(x);
    let x = triple(x);
    println!("x = {}", x);  // 가장 마지막에 shadowing된 값이 출력된다. 54가 출력된다.

    // shadowing-3
    let variable = 9;
    {
        let variable = "Some string";
        println!("variable1 = {}", variable); // variable1은 "Some string"이다.
    }
    println!("variable2 = {}", variable);   // variable1과 variable2는 다른 변수이다. 9가 출력된다.
}

fn double(input: i32) -> i32 {
    input * 2
}

fn triple(input: i32) -> i32 {
    input * 3
}

