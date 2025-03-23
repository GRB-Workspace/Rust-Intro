pub fn rust_ownership_intro() {
    // Ownership rules:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Stack and Heap:
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    println!("---------------------------------------------");

    // Move trait: (Move transfers the ownership)
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // This will cause an error because s1 is no longer valid

    // Clone trait: (Clone copies the heap data)
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("---------------------------------------------");

    // Copy trait: (Copy copies the stack data)
    // Types that are Copy:
    // 1. All the integer types, such as u32.
    // 2. The Boolean type, bool, with values true and false.
    // 3. All the floating point types, such as f64.
    // 4. The character type, char.
    // 5. Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.
    let x = 5;
    let y = x; // This is a shallow copy because x is a stack data
    println!("x = {}, y = {}", x, y);
    println!("---------------------------------------------");
}
