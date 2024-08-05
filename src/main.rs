fn main() {
    stack_allocation();
    heap_allocation();
    // stack_overflow(); uncomment to see stack overflow error
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
