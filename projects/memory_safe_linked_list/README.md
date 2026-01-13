# Memory-Safe Linked List in Rust

This project demonstrates a basic implementation of a singly linked list in Rust, emphasizing memory safety through Rust's ownership and borrowing system.

## Features

*   Creation of a linked list.
*   Appending elements to the list.
*   Popping elements from the list.
*   Iteration through the list.

## Usage

```rust
use memory_safe_linked_list::LinkedList;

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
```

## Contributing

Contributions are welcome! Please submit pull requests with clear descriptions of your changes.

## License

MIT