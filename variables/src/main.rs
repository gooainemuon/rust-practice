fn main() {
    // 변수
    let mut y = 5;
    println!("y : {y}");
    mut y = 6;
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
    
}
