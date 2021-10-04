// 3.2.3 Linked List

use crate::List::*;

// A box is a smart pointer to a heap allocated value of type T
// 라고 합니다. 아마 boxing + 스마트 포인터인듯합니다.
// 예제에는 List라고 되어있지만 Node라고 이해하는게 좋은거같습니다. Cons (현재 노드의 값, 다음 노드의 주소)
enum List {
    Cons(u32, Box<List>),
    Nil, // 리스트의 끝을 의미합니다. 즉 마지막 노드입니다.
}

// enum에도 이것저것들을 attach 할 수 있다.
impl List {
	
	// 생성자
    fn new() -> List {
        Nil
    }

	// 앞에 원소 추가
    fn prepend(self, elem: u32) -> List {
		// 현재 노드를 다음 노드로 하는 노드를 리턴해 줍니다.
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
		// 먼저 self의 값을 읽고 재귀적으로 다음 노드를 읽어옵니다.
        match *self {
			// 다음 노드가 있을경우 1을 더해주고 다음 노드를 호출합니다.
			// 여기서 ref인 이유는 tail에 대한 소유권을 가져올수 없기 때문에
			// 참조로 가져옵니다.
            Cons(_, ref tail) => 1 + tail.len(),
			// 마지막 노드에서 0을 반환합니다.
            Nil => 0
        }
    }

	fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 힙을 리턴
                // 문자열 할당
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
