// 5.2 Literals
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // 묵시적 형변환은 안됩니다.
    // let integer: u8 = decimal;

    // 명시적 형변환을 위해 as를 사용합니다.
    let integer = decimal as u8;
    let character = integer as char;
 
    // 허용되지 않는 형변환입니다.(float to char)
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type
	// 라고 적혀있는데, unsigned Type의 MAX+1은 casting 하면
	// 0이 됩니다. 즉 결과에 영향을 끼치지 못합니다.
	// 즉 A를 T로 castring한 결과가 B라고 치면
	// A = B + (T::Max + 1) * k 꼴이 되는것입니다.

    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);

	
	// signed value에서 최상위 비트가 1이면 음수입니다
	
	// 128 = 1000 0000
    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 = 1110 1000
    println!(" 232 as a i8 is : {}", 232 as i8);
    
    // Since Rust 1.45, the `as` keyword performs a *saturating cast* 
	// 라고 하는데 버림한다는 뜻입니다.
    
    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);
    
	// 강제 형변환인데 처리 비용이 있고, 값의 살짝 오류가 생길수도 있다고 합니다. 
    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
