fn main() {
    let mut llist = LinkedList::new(10);
    llist.push(11);
    llist.push(16);
    llist.print_list();
    llist.print_list();

    println!("Create a linked list with strings");
    let mut llist = LinkedList::new("A");
    llist.push("Another");
    llist.push("Something");
    llist.print_list();
    println!("Pop last item: {}", llist.pop().unwrap());
}

type NodePtr<T> = Option<Box<Node<T>>>;

struct Node<T: std::fmt::Display> {
    data: T,
    next: NodePtr<T>,
}

impl<T: std::fmt::Display> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            next: None,
        }
    }
    fn print_node(&self) {
        println!("Node value {}", self.data);
        match &self.next {
            Some(x) => &x.print_node(),
            None => return,
        };
    }
}

struct LinkedList<T: std::fmt::Display> {
    first: NodePtr<T>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    // New Linked List
    fn new(start_value: T) -> Self {
        let new_node = Box::new(Node::new(start_value));
        LinkedList {
            first: Some(new_node),
        }
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
