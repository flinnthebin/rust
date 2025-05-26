pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // recursive traversal accumulate values
    fn acc_recursive<'a>(node: &'a Option<Box<Node<T>>>, acc: &mut Vec<&'a T>) {
        if let Some(ref curr) = node {
            acc.push(&curr.value);
            Self::acc_recursive(&curr.next, acc);
        }
    }
    pub fn acc_refs(&self) -> Vec<&T> {
        let mut acc = Vec::new();
        Self::acc_recursive(&self.head, &mut acc);
        acc
    }

    // reverse a linked list
    pub fn reverse(self) -> LinkedList<T> {
        fn reverse_recursive<T>(
            node: Option<Box<Node<T>>>,
            acc: Option<Box<Node<T>>>,
        ) -> Option<Box<Node<T>>> {
            match node {
                Some(mut n) => {
                    let next = n.next.take(); // Take the next node
                    n.next = acc; // Point current node to the accumulator
                    reverse_recursive(next, Some(n)) // Recur with the next node and updated accumulator
                }
                None => acc, // Base case: return the accumulated reversed list
            }
        }
        LinkedList {
            head: reverse_recursive(self.head, None),
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(5);
    list.push(6);
    list.push(7);
    for value in list.acc_refs() {
        println!("{}", value);
    }
    let rev_list = list.reverse();
    for value in rev_list.acc_refs() {
        println!("{}", value);
    }
}
