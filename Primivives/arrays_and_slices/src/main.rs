// 2.3 Arrays and slices

use std::mem;


// call by ref
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("number of elements in array: {}", xs.len());

	
	// c++ 과 다르게 배열도 &로 ref를 잡아줘야 한다. 배열의 이름이 포인터가 아닌듯
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("array occupies {} bytes", mem::size_of_val(&ys));

	// println!("xs : {:}", xs); error 발생
    println!("xs : {:?}", xs); // 기본적으론 printalbe이 아니고 Debug가 정의되어 있다

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]); 

	
    // println!("{}", xs[5]);
}