#[derive(Debug)]
struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Insert at the end
    fn insert(&mut self, value: u32) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });

        match self.head.as_mut() {
            // Empty list
            None => {
                self.head = Some(new_node);
            }

            // Traverse till last node
            Some(mut current) => {
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }

                current.next = Some(new_node);
            }
        }
    }

    fn search(&self, value: u32) -> bool {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if node.value == value {
                return true;
            }

            current = node.next.as_ref();
        }

        false
    }

    fn print(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = node.next.as_ref();
        }

        println!("None");
    }
}

fn main() {
    let mut ll = LinkedList::new();

    ll.insert(10);
    ll.insert(20);
    ll.insert(30);
    ll.insert(40);

    ll.print();

    println!("{}", ll.search(20)); // true
    println!("{}", ll.search(100)); // false
}