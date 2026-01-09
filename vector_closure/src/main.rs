fn main(){
    let mut v:Vec<i32> = Vec::new();
    v.extend(1..5);
    println!("{:?}",v);
    
    let v2: Vec<i32> = (1..6).collect();
    println!("{:?}",v2);
    
    let v3 = vec![0;5];
    println!("{:?}",v3);
    
    //let arr = (3..15).collect(); 
    //배열은 스텍에 있어 컴파일 타임에 크기가 확정되어야한다.
    //루프를 돌며 아이템을 수집하는 콜랙터를 못쓴다
    let arr:[i32;12] = std::array::from_fn(|i|(i+3)as i32);
    //from_fn이 배열의 인덱스 0부터 돌리기 시작한다.
    let v4 = arr.to_vec();
    println!("{:?}",v4);

    let arr2: [i32;10] = std::array::from_fn(|i|{
        if i%2==0{
            i as i32
        }else{
            0
        }
    });

    println!("{:?}",arr2);

    let arr3: [i32;10] = std::array::from_fn(
        |i| match i%2{
            0 => i as i32, // 나머지가 0이면 값 반환
            _=>0, // 아니면 0
        }
    );
    println!("{:?}",arr3);

}