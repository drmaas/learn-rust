//! Week 8 — Smart pointers, lifetimes deep dive, memory model, unsafe-lite.

use std::cell::RefCell;
use std::rc::Rc;

/// A recursive type needs `Box` (one layer of indirection) to have a finite
/// size. `enum List { Cons(i32, List) }` would be infinitely sized.
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn list_length(list: &List) -> u32 {
    match list {
        List::Cons(_, rest) => 1 + list_length(rest),
        List::Nil => 0,
    }
}

/// `Rc` shares ownership; `Rc::clone` increments the count (not a deep copy).
pub fn rc_strong_count() -> usize {
    let a = Rc::new(5);
    let _b = Rc::clone(&a);
    Rc::strong_count(&a)
}

/// `Rc<RefCell<Node>>` — shared ownership AND interior mutability.
pub struct Node {
    pub name: String,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(name: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            name: name.to_string(),
            neighbors: Vec::new(),
        }))
    }
}

pub fn add_edge(a: &Rc<RefCell<Node>>, b: &Rc<RefCell<Node>>) {
    a.borrow_mut().neighbors.push(Rc::clone(b));
}

/// A struct holding a reference needs a lifetime parameter.
pub struct Slice<'a> {
    pub data: &'a [u8],
}

impl<'a> Slice<'a> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// `Drop` / RAII: observe drop order. Variables drop in reverse declaration
/// order (LIFO), so the log is `["b", "a"]`.
pub fn drop_order() -> Vec<&'static str> {
    struct Named(&'static str, Rc<RefCell<Vec<&'static str>>>);
    impl Drop for Named {
        fn drop(&mut self) {
            self.1.borrow_mut().push(self.0);
        }
    }

    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _a = Named("a", Rc::clone(&log));
        let _b = Named("b", Rc::clone(&log));
    }
    // Bind the guard to a variable: a temporary in a tail expression would
    // otherwise live until the end of the block, outliving `log`.
    let out = log.borrow().clone();
    out
}

/// Niche optimization: `Option<&T>` is the same size as `&T` (null = `None`).
pub fn option_ref_is_free() -> bool {
    std::mem::size_of::<Option<&u8>>() == std::mem::size_of::<&u8>()
}

/// `Option<u8>` has NO niche, so it needs a tag byte -> 2 bytes.
pub fn size_of_option_u8() -> usize {
    std::mem::size_of::<Option<u8>>()
}

/// A tiny custom smart-pointer-ish type (Week 8 Day 5).
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(v: T) -> MyBox<T> {
        MyBox(v)
    }
    pub fn get(&self) -> &T {
        &self.0
    }
}

/// The memory-diagram program: owned String, a reference, and a Box.
pub fn memory_diagram() -> String {
    let s = String::from("hello");
    let r = &s;
    let b = Box::new(42);
    format!("{r} {b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_list_with_box() {
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        assert_eq!(list_length(&list), 2);
    }

    #[test]
    fn rc_shares_ownership() {
        assert_eq!(rc_strong_count(), 2);
    }

    #[test]
    fn refcell_graph_mutation() {
        let a = Node::new("a");
        let b = Node::new("b");
        add_edge(&a, &b);
        assert_eq!(a.borrow().neighbors.len(), 1);
        a.borrow_mut().name = "A".to_string();
        assert_eq!(a.borrow().name, "A");
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn refcell_double_borrow_panics() {
        // The course's "deliberately cause a borrow() panic" exercise — the
        // panic IS the lesson, so this is a real #[should_panic] test.
        let cell = RefCell::new(5);
        let _shared = cell.borrow();
        let _mutable = cell.borrow_mut(); // panics: already borrowed
    }

    #[test]
    fn slice_struct_holds_reference() {
        let data = [1u8, 2, 3];
        let s = Slice { data: &data };
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn drop_order_is_lifo() {
        assert_eq!(drop_order(), vec!["b", "a"]);
    }

    #[test]
    fn niche_optimization_sizes() {
        assert!(option_ref_is_free());
        assert_eq!(size_of_option_u8(), 2);
        assert_eq!(std::mem::size_of::<u8>(), 1);
    }

    #[test]
    fn mybox_get() {
        let b = MyBox::new(42);
        assert_eq!(*b.get(), 42);
    }

    #[test]
    fn memory_diagram_prints() {
        assert_eq!(memory_diagram(), "hello 42");
    }
}
