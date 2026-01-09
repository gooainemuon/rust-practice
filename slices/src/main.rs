fn main() {
    // 배열 슬라이스
    let arr = [1, 2, 3, 4, 5];
    
    // 슬라이스 생성
    let arr1 = &arr[1..3];
    let arr3 = &arr[..3];
    let arr2 = &arr[2..];

    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    println!("arr3: {:?}", arr3);

    // 원본 배열
    println!("arr: {:?}", arr);

    // 문자열 슬라이스 - 가변
    let mut s = String::from("String Slice");
    let s1 = &s[0..6];
    let s2 = &s[7..];
  
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s: {}", s);

    s.push_str("변경가능");
    println!("s: {}", s);

    // 바이트단위의 문자열 길이
    println!("UTF-8 byte s.len(): {}", s.len());

    // 문자열 슬라이스 - 불변
    let vs = "str Slice";
    let vs1 = &vs[0..3];
    let vs2 = &vs[4..];

    println!("vs1: {}", vs1);
    println!("vs2: {}", vs2);
    println!("vs: {}", vs);

    
}
