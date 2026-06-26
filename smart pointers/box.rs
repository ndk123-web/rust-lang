struct ListNode {
    value: u8,
    next: Option<Box<ListNode>>,
}

fn main() {
    {
        let a = Box::new(10);
        println!("{}", a);
    }

    // box destroyed when scoped ends
    // println!("{}", a);

    let u1: ListNode = ListNode {
        value: 10,
        next: None,
    };

    let u2: ListNode = ListNode {
        value: 20,
        next: Some(Box::new(u1)),
    };

    println!("{}", u2.next.unwrap().value);
}
