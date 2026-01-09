fn main() {
    //변수에 바로 넣는 스위치문!
    let score = 85;
    let grade = match score{
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F'
    };
    println!("Your score {}, grade is {}", score, grade);

    //이런것도 가능하다!
    let dice = 3;
    match dice {
    /*  '|'
        쉘     : 결과를뒤로 넘기는 흐름
        러스트  : 이들중 선택
    */
        1|3|5 => println!("홀수"),
        2|4|6 => println!("짝수"),
        _=>println!("?")
    }

    //늘 swich에서 꿈꿔오던 안에 if문 넣는게 가능하다
    let num =75;
    match num {
        x if (0..=100).contains(&x) && x%2==0
        =>println!("{}는 짝수", x),
        x if (0..=100).contains(&x) && x%2==1
        =>println!("{}는 홀수", x),
        _=>println!("?")
    }
}
