fn main() {
    let stack_var = 10;

    // Box points to data on the heap
    // Most straightforward way to put data on the heap
    let heap_var = Box::new(20);

    // The Vec itself is on the stack, holding capacity, length, and a pointer to its data 
    // The Vec data is stored on the heap for dynamic sizing
    // This is similar to how Rc and Arc work; the metadata is on the stack while the data is on the heap
    let mut heap_vec = Vec::new();
    heap_vec.push(30);
    heap_vec.push(40);
    heap_vec.push(50);

    // Reference Counted for shared ownership of a variable
    // The Rc holds a reference count and its incremented on every clone
    // When the count drops to zero, the data is deallocated
    use std::rc::Rc;
    let rc_heap_var = Rc::new(60);
    let _rc_clone_1 = Rc::clone(&rc_heap_var); // underscore unused vars
    let _rc_clone_2 = Rc::clone(&rc_heap_var); // Increments ref count to 3

    // Atomic Reference Counted for thread-safe ownership
    // Same as Rc but for working with multiple threads
    use std::sync::Arc;
    let arc_heap_var = Arc::new(70);
    let _arc_clone_1 = Arc::clone(&arc_heap_var); // Increments ref count to 2

    // A "cell" is a construct that allows controlled mutable access to an immutable container
    // The RefCell is our immutable container in this case; it maintains the value's location and borrow counts
    // Case for interior mutability: when a program has shared state but you want to control and restrict how the data is accessed and modified  
    use std::cell::RefCell;
    let refcell_heap_var = RefCell::new(80);
    {
        // borrow_mut() returns a mutable reference to the RefCell value
        let mut mutable_ref = refcell_heap_var.borrow_mut();
        // mutable_ref is a direct reference to the value inside refcell_heap_var
        *mutable_ref += 10;
    }

    // A Cell allows for interior mutability with types that implement the Copy trait
    // It provides a simpler way to mutate values without runtime borrow checking
    use std::cell::Cell;
    let cell_heap_var = Cell::new(100);
    // Directly set a new value 
    cell_heap_var.set(110);
    // Get the current value 
    let cell_value = cell_heap_var.get();

    println!("Data on the stack: {}", stack_var);
    println!("Data in the Box on the heap: {}", heap_var);
    println!("Data in the Vec on the heap: {:?}", heap_vec);

    println!("Reference counted heap variable: {}, with a reference count of {}", *rc_heap_var, Rc::strong_count(&rc_heap_var));

    println!("Atomic reference counted heap variable: {}, with a reference count of {}", *arc_heap_var, Arc::strong_count(&arc_heap_var));
    println!("Mutated RefCell heap variable: {}", refcell_heap_var.borrow());
    println!("Mutated Cell heap variable: {}", cell_value);
}
