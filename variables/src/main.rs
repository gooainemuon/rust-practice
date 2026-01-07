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
    
    // 부동 수소점
    let my_f64 = 2.0;
    let my_f32: f32 = 3.0; 
    println!("my_f64 : {my_f64}");
    println!("my_f32 : {my_f32}");


}
