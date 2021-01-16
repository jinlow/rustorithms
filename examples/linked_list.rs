use linked_list::LinkedList;

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

    println!("Create a linked list from a vector");
    let llist = LinkedList::from_vec(vec!["vector", "a", "is", "this"]);
    llist.print_list();
}
