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

/// Stack data structure implemented as a linked list.
pub struct LinkedList<T: std::fmt::Display> {
    first: NodePtr<T>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    /// New Linked List
    pub fn new() -> Self {
        LinkedList { first: None }
    }

    /// Linked List from a vector
    pub fn from_vec(x: Vec<T>) -> Self {
        let mut ll = LinkedList{first: None};
        for item in x {
            ll.push(item);
        }
        return ll
    }

    /// Push data onto front of list
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            next: self.first.take(),
        });
        self.first = Some(new_node);
    }

    /// Print the list
    pub fn print_list(&self) {
        if let Some(x) = &self.first {
            x.print_node();
        };
    }
    /// Pop item off of list
    pub fn pop(&mut self) -> Option<T> {
        match self.first.take() {
            Some(x) => {
                self.first = x.next;
                return Some(x.data);
            }
            _ => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_push_pop_string() {
        let mut ll = LinkedList::new();
        ll.push("what's");
        ll.push("that?");
        assert_eq!(ll.pop().unwrap(), "that?");
    }

    #[test]
    fn test_push_pop_int() {
        let mut ll = LinkedList::new();
        ll.push(100);
        ll.push(4);
        assert_eq!(ll.pop().unwrap(), 4);
    }

    #[test]
    fn test_from_vec() {
        let mut ll = LinkedList::from_vec(vec![1, 2, 3, 4, 2, 5]);
        assert_eq!(ll.pop().unwrap(), 5);
    }
}
