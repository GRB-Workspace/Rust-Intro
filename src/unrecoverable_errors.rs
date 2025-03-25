pub fn unrecoverable_errors_intro() {
    /*
    Unrecoverable Errors with panic! macro
     2 ways to invoke panic! macro
         01. Calling panic! macro directly / intentionally
         02. When Rust detects an unrecoverable error
    */

    // 01. Calling panic! macro directly / intentionally
    //panic!("Crash and Burn!");

    // 02. When Rust detects an unrecoverable error
    let v = vec![1, 2, 3];
    // v[99]; // This will cause a panic!
}
