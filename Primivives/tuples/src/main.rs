// 2.2 tuples
use std::fmt::{self, Formatter, Display};

fn reverse(pair: (i32, bool))-> (bool, i32){
	let (integer, boolean) = pair;
	
	(boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Activity 1
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		// 왜 여기는 ;을 안쓸까
        write!(f, "({:.1} {:.1})\n({:.1} {:.1})", self.0, self.1, self.2, self.3)
    }
}


// Activity2
fn transpose(rhs : Matrix) -> Matrix{
	Matrix(rhs.0,rhs.2,rhs.1,rhs.3)
}

fn main(){
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
	
	// 튜플의 인덱스를 [idx]가 아닌 tuple.idx 형식으로 가져온다
	println!("long tuple first value : {}", long_tuple.0);
	println!("long tuple first value : {}", long_tuple.1);
	
	
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    // 너무 길면 출력 안됨
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // /println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // 쉼표를 넣어 줘야 원소 한개짜리 튜플이 생성된다.
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // println!("{:?}", matrix); // Debug
	println!("Matrix:\n{}", matrix); // Display(Activity1)
	println!("Transpose:\n{}", transpose(matrix)); //Transpose(Activity2)
}


