fn main() {
    println!("Hello, world!");
    // let value = serde_json::json!({
    //     "code": 200,
    //     "success": true,
    //     "payload": {
    //         "features": [
    //             "serde",
    //             "json"
    //         ]
    //     }
    // });
    //
    // println!("json = {:?}", value);

    // 간단한 정수 계산기
    let program = "+ + * - /";
    let mut accumulator = 5;

    for token in program.chars() {
        match token {
            '+' => accumulator += 1,
            '-' => accumulator -= 1,
            '*' => accumulator *= 2,
            '/' => accumulator /= 2,
            _ => { /* 다른 문자들 무시 */ }
        }
    }

    println!("프로그램 \"{}\"의 계산값은 {}입니다.", program, accumulator);
}
