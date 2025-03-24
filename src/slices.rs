pub fn slices_intro() {
    // slice of an array of characters
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &arr[1..3];
    println!("{:?}", slice);
    println!("---------------------------------------------");

    // slice of a vector of integers
    // vectors are dynamic arrays (resizable arrays)
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &vec[1..3];
    println!("{:?}", slice);
    println!("---------------------------------------------");

    // slices for Strings
    let s: String = String::from("Hello, World!");
    let slice: &str = &s[0..5];
    println!("{:?}", slice);
    println!("---------------------------------------------");

    // Ranges shortcut for slices
    let s = String::from("Hello, World!");

    // Shortcut for initial index
    let slice: &str = &s[..5];
    println!("{:?}", slice);

    // Shortcut for final index
    let slice: &str = &s[7..];
    println!("{:?}", slice);
    let slice: &str = &s[7..s.len()];
    println!("{:?}", slice);

    // Shortcut for both initial and final index
    let slice: &str = &s[..];
    println!("{:?}", slice);
    let slice: &str = &s[0..s.len()];
    println!("{:?}", slice);
    println!("---------------------------------------------");

    // String literals are slices
    let s: &str = "Hello, World!";
    println!("{:?}", s);
    println!("---------------------------------------------");
}
