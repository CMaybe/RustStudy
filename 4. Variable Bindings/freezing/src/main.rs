fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // 여기에서의 _mutable_integer은 mut이 아닙니다.
        // _mutable_integer = 50;

    }

    _mutable_integer = 3;
	println!("_mutable_integer: {}", _mutable_integer);
}
