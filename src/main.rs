use std::{cell::RefCell, rc::{Rc, Weak}};

struct Person {
    name: String,
    // Children strongly owned by parent
    children: RefCell<Vec<Rc<Person>>>,
    // Parent weakly referenced by child
    parent: RefCell<Weak<Person>>
}

fn main() {
    // Create a parent
    let parent = Rc::new(Person {
        name: "Mom".to_string(),
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    // Create a child with a weak reference to parent
    let child = Rc::new(Person {
        name: "Kid".to_string(),
        children: RefCell::new(vec![]),
        parent: RefCell::new(Rc::downgrade(&parent))
    });

    let data = Rc::new(42);
    let strong_ref1 = Rc::clone(&data);  // strong_count = 2
    let weak_ref1 = Rc::downgrade(&data); // weak_count = 1

    // Check if weak reference still points to valid data
    if let Some(value) = weak_ref1.upgrade() {
        println!("Data still exists: {}", value);
    }
}