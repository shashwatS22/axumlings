// Interior mutability: use RefCell to mutate through an immutable reference.
//
// Hint: RefCell::new(value), .borrow_mut() for mutable access

use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(0);
    // TODO: Use cell.borrow_mut() to set the value to 42
    println!("{}", *cell.borrow());
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    #[test]
    fn test_refcell() {
        let cell = RefCell::new(0);
        *cell.borrow_mut() = 42;
        assert_eq!(*cell.borrow(), 42);
    }
}
