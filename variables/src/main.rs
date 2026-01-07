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

    // 변수 = 연산
    let _sum = 5+10;
    let _diff = 95.5-4.3;
    let _product = 4*30;
    let _quotient = 56.7/32.2;
    let _remainder = 43 % 5;

    // 불타입
    let t = true;
    let f: bool = false;
    println!("t : {t}");
    println!("f : {f}");

    // char
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c : {c}");
    println!("z : {z}");
    println!("heart_eyed_cat : {heart_eyed_cat}");

    // tuple - 복합타입, 길이 불변, 요소 타입 달라도 됨
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x : {x}");
    println!("y : {y}");
    println!("z : {z}");

    let _x = tup.0;
    let _y = tup.1;
    let _z = tup.2;

    // array - 배열, 길이 불변, 요소타입 동일, 스택에 할당
    //let a = [1, 2, 3, 4, 5];
    let a = [i32; 5];
    println!("a : {a:?}");

}
