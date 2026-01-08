/*
  소유권: 메모리주소의 데이터에 대한  수정(읽기 쓰기 삭제)권한
  소유권 이동: 권한 완전 양도
  소유권 대여: 참조
  불변대여(&T): 읽기만 가능
  가변대여(&mut T): 읽기+쓰기

  1. 배열 슬라이스: &[T]     → [1,2,3][1..3] = &[2,3]
  2. 스트링 슬라이스: &str   → "hello"[0..2] = "he"  
  3. 문자열 슬라이스: &String → &s[..] 전체 참조

*/
fn main() {
    // 1. 소유권 이동 권한 완전양도
    let s1 = String::from("hello"); // s1이 소유권 획득
    let s2 = s1;                    // s2로 권한 완전 이동
    //println!("{}", s1);             //  컴파일 에러
    println!("{}", s2);             //  컴파일 에러

    // 2. 불변 대여. 읽기 권한만 위임
    let mut c_loan = String::from("불변대여");
    let c_loan_01 = &c_loan; // 여러곳에 읽기 권한 대여 가능
    let c_loan_02 = &c_loan;
    println!("c_loan_01: {}, c_loan_02: {}", c_loan_01, c_loan_02);

    c_loan.push_str(" world"); // 에러. 읽기 대여중 쓰기 불가
    println!("c_loan: {}", c_loan); // 뭐야 왜 애러 안나? -> println!이후 안쓰여서 스코프 종료됨
    
    // 3. 가변 대여. 읽기+쓰기 권한 위임
    let mut v_loan = String::from("가변대여");
    let v_loan_01 = &mut v_loan;
    // v_loan.push_str(" world"); // 에러 성공 소유권 중복할당 불가
    v_loan_01.push_str(" world");
    println!("v_loan_01: {}", v_loan_01);

    // 불변대여와 가변대여 동시에 줄 수 없음  - 가변대여변수가 변경중에 불변대여변수가 읽는 경우 방지    
}
