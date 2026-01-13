mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("List: {:?}", list);

    while let Some(value) = list.pop() {
        println!("Popped: {}", value);
    }

    println!("List after popping: {:?}", list);
}