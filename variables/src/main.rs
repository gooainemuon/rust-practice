fn main() {
    // 변수
    let mut y = 5;
    println!("y : {y}");
    y = 6;
    println!("y : {y}");

    let x = 5;
    println!("x : {x}");
    let x = x + 1;
    {
        let x = "string";
        println!("x : {x}");
    }
    println!("x : {x}");

    // 상수
    const MYCONST: i32 = 100;
    println!("MYCONST : {MYCONST}");

    // data type - 뭐지 사이트에서는 안된다고 헀는데 된다.
    let _guess: u32 = "42".parse().expect("Not num");
    
    // 스칼라타입: 단일값
}
