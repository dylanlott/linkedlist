use std::{cell::RefCell, rc::Rc};

// pub struct Payload<T> {
//     value: T
// }
struct ListNode<T> {
    // item: Payload<T>,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> ListNode<T> {
    fn new(_: T) -> Self {
        Self{
            // item: Payload{ value: item},
            next: None,
            prev: None,
        }
    }
    // fn get(self) -> Payload<T> {
    //     return self.item
    // }
}

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

#[derive(Default)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self{
            head: None,
            tail: None,
            size: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn push_back(&mut self, item: T) {
        // we make a new ListNode out of our item T and wrap it in a Reference Counter
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        // we check if self tail exists with `.take()`
        if let Some(prev_tail) = self.tail.take() {
            // then we want to give to the last tail. 
            // so we borrow a reference to the previous tail and set it's next to the new next node
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            // set the new node's previous node to the prev_tail we took
            node.borrow_mut().prev = Some(prev_tail);
            // increase size by one since we added a node
            self.size += 1;
        } else {
            // if we can't take from the tail, then we're at the start of the list. 
            // so we assign the head to this new node by cloning a strong reference to it.
            self.head = Some(Rc::clone(&node));
            // and then we set the tail to that node
            self.tail = Some(node);
            self.size = 1;
        }
    }
    pub fn push_front(&mut self, item: T) {
        // we make a new ListNode out of our item T and wrap it in a Reference Counter
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        // similar to push_back, we check for the existence of the head
        if let Some(prev_head) = self.head.take() {
            prev_head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(prev_head);
            self.head = Some(node);
            self.size += 1;
        } else {
            // if 
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    // because we have cyclic references in our code, we need to handle our own cleanup 
    // by manually declaring the drop trait for our doubly linked list.
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    assert_eq!(list.is_empty(), true);

    let node = ListNode::new(1);
    let node2 = ListNode::new(2);

    list.push_back(node);
    list.push_back(node2);

    assert_eq!(list.len(), 2);
}
