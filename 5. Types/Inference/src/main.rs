//  5.3 Inference

fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();
	
	// vec에 u8 type의 elem을 넣어줌으로써 vec의 타입이 결정됩니다.
	// 밑에 줄을 주석처리하면 vec의 타입이 결정되지 않기 때문에 에러가 발생합니다. 	
    vec.push(elem);

    println!("{:?}", vec);
}