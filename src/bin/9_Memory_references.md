# The stack, the heap, and pointers

---

### Reference 의 사용 핵심 차이점은 _참조(reference)_ 와 _값(value)_ 사이의 차이입니다.

---

## 1. ✅ 첫 번째 코드: 참조를 이용한 다중 참조 (Reference 사용)
```rust
fn main() {
let my_number = 15;
let single_reference = &my_number;  // &my_number -> my_number의 주소를 가리키는 참조
let double_reference = &single_reference;   // &(&my_number) -> 참조를 가리키는 참조
let five_reference = &&&&&my_number;    // 여러 단계의 참조
}
```
<br>

**✔️ 메모리 구조:**

| 변수 이름              | 저장된 값               | 의미                          |
|--------------------|---------------------|-----------------------------|
| `my_number`        | `15`                | 실제 정수 값                     |
| `single_reference` | `&my_number`        | `my_number`를 가리키는 참조        |
| `double_reference` | `&single_reference` | `single_reference`를 가리키는 참조 |
| `five_reference`   | `&&&&&my_number`    | `my_number`의 다중 참조          |

**즉, `five_reference`는 `my_number`의 다섯 단계 참조입니다.**

**💡 이 코드의 특징:** <br>
* `&my_number`는 `my_number`의 주소를 가리키는 참조입니다.
* `&&&&&my_number`는 다섯 단계의 참조입니다.
* 참조를 사용하므로, `my_number`는 소유권을 잃지 않습니다.

<br>

## 2. ❌ 두 번째 코드: 값을 복사 (Value Copy)
```rust
fn main() {
let my_number = 15;
let single_reference = my_number;  // 그냥 값을 복사
let double_reference = single_reference;   // 값 복사
let five_reference = my_number;  // 값 복사
}
```
<br>

**✔️ 메모리 구조:**

| 변수 이름              | 저장된 값 | 의미                       |
|--------------------|-------|--------------------------|
| `my_number`        | `15`  | 실제 정수 값                  |
| `single_reference` | `15`  | `my_number` 값을 복사        |
| `double_reference` | `15`  | `single_reference` 값을 복사 |
| `five_reference`   | `15`  | `my_number` 값을 복사        |

**💡 이 코드의 특징:** <br>
* 참조가 아닌 값 자체를 복사합니다.
* `my_number`, `single_reference`, `double_reference`, `five_reference는` 모두 서로 독립적인 값입니다.
* `my_number`의 소유권은 유지되며, 참조와 다르게 직접 값을 복사하므로 여러 개의 독립적인 변수가 같은 값을 가집니다.

---

---

---

## 🎯 핵심 차이점

|                        | reference 사용 (첫 번째 코드) | Value copy (두 번째 코드)    |
|------------------------|------------------------|-------------------------|
| 값의 소유권                 | `my_number`가 소유권 유지    | 값이 복사됨 (새로운 소유권)        |
| 메모리 사용                 | 참조만 저장 (포인터 크기만큼)      | `my_number` 값이 여러 번 저장됨 |
| 다중 참조 가능 여부            | 가능 (`&&&&&my_number`)  | 불가능 (값만 저장)             |
| `single_reference`의 타입 | `&i32` (참조)            | `i32` (값)               |

<br>

## ❗ 결론

``` rust
# Rust 코드를 직접 실행할 수 없으므로 Python으로 유사한 동작을 시뮬레이션하여 차이를 보여줌

# 첫 번째 코드 (reference 사용)
def first_code():
my_number = 15
single_reference = my_number  # Python에서는 참조와 값 복사가 구별되지 않음
double_reference = single_reference
five_reference = my_number

    return my_number, single_reference, double_reference, five_reference

# 두 번째 코드 (value 복사)
def second_code():
my_number = 15
single_reference = my_number
double_reference = single_reference
five_reference = my_number

    return my_number, single_reference, double_reference, five_reference

# 실행 결과 비교
first_result = first_code()
second_result = second_code()

first_result, second_result
```

``` sql
결과
((15, 15, 15, 15), (15, 15, 15, 15))
```

두 코드의 실행 결과는 (15, 15, 15, 15)로 동일하게 보입니다. 하지만 Rust에서의 동작은 Python과 다릅니다. <br>
Python에서는 변수 할당이 항상 참조 기반이기 때문에 두 코드의 차이가 드러나지 않습니다. Rust에서는 참조와 값 복사가 엄격히 구분되기 때문에 메모리에서 다르게 동작합니다.

### 👉 따라서, 두 코드의 출력 결과는 같아 보일 수 있지만, 메모리 사용과 동작 방식이 다릅니다!
* 첫 번째 코드에서는 my_number의 참조를 계속 연결했기 때문에, 메모리에 **포인터(참조) 크기(8 bytes)**가 저장됩니다.
* 두 번째 코드에서는 값이 복사되었기 때문에, 변수마다 **4 bytes (i32 크기)**가 차지됩니다.
* Rust에서는 참조를 사용할 경우 소유권(ownership)이 유지되지만, 값을 복사하면 새로운 값이 생성됩니다.<br>

** _즉, 결과적으로 같은 값이 나오지만, 내부 동작 방식과 메모리 사용이 다르므로 "같다"라고 할 수 없습니다! 🚀_** 

___

___

___
## 🚨 그래서 결과는 같지만 위의 두 가지 방법 중 더 나은 성능은?

### 1. ✅ 첫 번째 코드(Reference 사용)의 장점과 단점
**🔹 장점:**<br>
1. 큰 데이터 구조를 효율적으로 다룰 수 있음
    * &T를 사용하면 값을 복사하지 않고 원본 데이터를 직접 가리킬 수 있음.
    * 따라서 큰 데이터(예: Vec<T>, HashMap<K, V>, String 등)를 사용할 경우 성능 최적화에 도움이 됨.
2. 소유권 유지 & 안전성
   * 첫 번째 코드에서는 my_number의 소유권을 유지하면서 여러 번 참조 가능.
   * 소유권을 잃지 않고 여러 함수에서 사용할 경우 유용함.<br>
   
**🔹 단점:** <br>
1. 참조 해제 비용 (Dereferencing Overhead)
   * &i32처럼 작은 타입에서는 포인터를 따라가는(dereference) 비용이 추가됨.
   * 즉, 직접 값을 복사하는 것보다 참조를 따라가는 과정이 오히려 더 느릴 수 있음.
2. 라이프타임(Lifetime) 관리 필요
   * Rust에서는 참조를 사용할 때 라이프타임('a)을 명시적으로 설정해야 하는 경우가 있음.
   * 단순 코드에서는 문제가 없지만, 복잡한 구조에서는 라이프타임 때문에 오히려 불편할 수 있음.

### 2. ✅ 두 번째 코드(Value 복사)의 장점과 단점
**🔹 장점:**<br>
1. 메모리 사용량 감소
   * 두 번째 코드에서는 값(i32)을 직접 저장하므로, 추가적인 참조(포인터) 저장 비용이 없음.
   * 즉, 스칼라 타입(예: i32, bool, f64)의 경우 복사 비용이 거의 없으므로 효율적임.
2. 캐시 친화적 (Cache Friendly)
   * 값이 메모리에서 연속적으로 저장되기 때문에, CPU 캐시 히트율이 높아질 가능성이 있음.
   * 따라서 성능 최적화가 필요한 경우 작은 크기의 단순 값은 복사하는 것이 더 빠름.

**🔹 단점:** <br>
1. 큰 데이터 타입에서 비효율적
   * i32처럼 작은 값은 복사 비용이 적지만, String, Vec\<T\> 같은 큰 데이터 타입을 복사하면 성능이 크게 저하됨.
   * Rust에서 Copy 트레이트가 없는 타입(Vec\<T\>, String, HashMap\<K, V\> 등)은 복사가 아니라 **소유권 이동(move)**이 일어나므로 예상치 못한 성능 저하 가능성이 있음.

---

### 🚀 결론: 성능 최적화 전략
1. 작은 크기의 스칼라 타입 (i32, f64, bool) → 값 복사가 더 빠름
   * 두 번째 코드 (let x = y;)처럼 그냥 값 복사하는 것이 더 효율적.
   * CPU 캐시 친화적이므로 성능이 향상될 가능성이 높음.
2. 큰 데이터 구조 (String, Vec<T>, HashMap<K, V>) → 참조가 더 빠름
   *  첫 번째 코드처럼 참조(&T)를 사용하는 것이 성능상 이점이 있음.
   * 값이 크면 복사 비용이 높아지므로 참조를 활용하는 것이 최적화에 유리함.
3. 다중 참조(&&&&T) 남발 금지
   *  &&&&&my_number 같은 다중 참조는 실제 코드에서 거의 사용되지 않음.
   * 참조를 여러 번 중첩하는 것은 가독성을 해치며, 성능상의 이점도 없음.

---

### 📌 정리

| 데이터 유형                            | 복사(Copy)가 빠름 | 참조(&T)가 빠름 |
|-----------------------------------|--------------|------------|
| i32, f64, bool                    | ✅            | ❌          |
| String, Vec\<T\>, HashMap\<K, V\> | ❌            | ✅          |
| struct (작은 크기)                    | ✅            | ❌          |
| struct (큰 크기)                     | ❌            | ✅          |
**_💡 즉, Rust에서는 "작은 값이면 복사하고, 큰 값이면 참조"하는 것이 일반적인 성능 최적화 방법입니다. 🚀_**
