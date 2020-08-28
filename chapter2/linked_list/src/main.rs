fn main() {
    let mut llist = LinkedList::new();
    llist.push(10);
    llist.push(11);
    llist.push(16);
    llist.print_list();

    println!("Create a linked list with strings");
    let mut llist = LinkedList::new();
    llist.push("A");
    llist.push("Another");
    llist.push("Something");
    llist.print_list();
    println!("Pop last item: {}", llist.pop().unwrap());
    llist.print_list();
}

type NodePtr<T> = Option<Box<Node<T>>>;

struct Node<T: std::fmt::Display> {
    data: T,
    next: NodePtr<T>,
}

impl<T: std::fmt::Display> Node<T> {
    fn print_node(&self) {
        println!("Node value {}", self.data);
        match &self.next {
            Some(x) => &x.print_node(),
            None => return,
        };
    }
}

// Stack data structure implemented as a linked list.
struct LinkedList<T: std::fmt::Display> {
    first: NodePtr<T>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    // New Linked List
    fn new() -> Self {
        LinkedList { first: None }
    }
    // Push data onto front of list
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            next: self.first.take(),
        });
        self.first = Some(new_node);
    }
    // Print the list
    fn print_list(&self) {
        if let Some(x) = &self.first {
            x.print_node();
        };
    }
    // Pop item off of list
    fn pop(&mut self) -> Option<T> {
        match self.first.take() {
            Some(x) => {
                self.first = x.next;
                return Some(x.data);
            }
            _ => return None,
        }
    }
}
