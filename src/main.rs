use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    stack_allocation();
    heap_allocation();
    // stack_overflow(); // uncomment to see stack overflow error
    // heap_overflow(); // uncomment to see heap overflow (ctrl + c to exit or terminate process)
    // store_large_data_heap(); // uncomment to see large data on heap
    // shared_ownership(); // uncomment to see shared ownership
    // arc_multi_thread(); // uncomment to see arc multi thread
    // raw_pointers(); // uncomment to see raw pointers (unsafe)

    // uncomment to see explicit lifetimes
    // let string1 = String::from("long string");
    // let string2 = String::from("short");
    //
    // let result = explicit_lifetimes(&string1, &string2);
    // println!("The longest string is: {}", result);
}

fn stack_allocation() {
    let x = 5;
    let y = x;
    println!("Stack allocation: x = {}, y = {}", x, y);
}

fn heap_allocation() {
    let x = Box::new(5);

    println!("Heap allocation: x = {}", x);
}

fn stack_overflow() {
    let large_array = [0u8; 10_000_000];
    println!("Large array on stack: {:?}", large_array);
}

fn heap_overflow() {
    let large_array = vec![0u8; 10_000_000];
    println!("Large array on heap: {:?}", large_array);
}

fn store_large_data_heap() {
    // Directly allocate a large vector on the heap
    let large_data = vec![0u8; 10_000_000]; // Vector allocated on the heap
    println!("Large data length: {}", large_data.len());
}

fn shared_ownership() {
    let shared_value = Rc::new(42);

    let clone_a = Rc::clone(&shared_value);
    let clone_b = Rc::clone(&shared_value);

    println!("Value: {}", shared_value);
    println!("Reference Count: {}", Rc::strong_count(&shared_value));
}

fn arc_multi_thread() {
    let shared_data = Arc::new(42);

    let threads: Vec<_> = (0..10).map(|_| {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            println!("Shared value: {}", data);
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }
}

fn raw_pointers() {
    let x = 42;
    let r = &x as *const i32; // Create a raw pointer

    unsafe {
        println!("Raw pointer dereference: {}", *r); // Dereference the raw pointer
    }
}

fn explicit_lifetimes<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
