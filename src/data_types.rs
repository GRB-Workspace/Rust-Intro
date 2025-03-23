pub fn data_types_intro() {
    /*
    Scaler types
    Rust has four primary scalar types:
     - Integers,
     - Floating-point numbers,
     - Booleans,
     - Characters.

    Compound types
    Rust has two primitive compound types:
     - Tuples,
     - Arrays.

    Custom types
    Rust also has the ability to define custom types using
      - structs and
      - enums.

    Integers
    Rust has several integer types:
    Length	Signed	Unsigned
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize
    */
    let small_number: u8 = 127;
    let big_number: u64 = 9223372036854775807;
    let small_number2: i8 = -128;
    let big_number2: i64 = -9223372036854775808;
    println!("The value of small_number is: {}", small_number);
    println!("The value of big_number is: {}", big_number);
    println!("The value of small_number2 is: {}", small_number2);
    println!("The value of big_number2 is: {}", big_number2);
    println!("---------------------------------------------");

    /*
    Numeral         System  Description      Example
    Decimal         Base 10 The common form  98_222
    Hexadecimal     Base 16 Prefixed with 0x 0xff
    Octal           Base 8  Prefixed with 0o 0o77
    Binary          Base 2  Prefixed with 0b 0b1111_0000
    Byte (u8 only)  Base 2  Byte (8 bits)    b'A'
    */
    let decimal_number: f64 = 98_222.0;
    let hex_number = 0xff;
    let octal_number = 0o77;
    let binary_number = 0b1111_0000;
    println!("The value of decimal_number is: {}", decimal_number);
    println!("The value of hex_number is: {}", hex_number);
    println!("The value of octal_number is: {}", octal_number);
    println!("The value of binary_number is: {}", binary_number);
    println!("---------------------------------------------");

    /*
    Floating-Point Numbers
    Rust has two primitive floating-point types:
    f32 and f64. These types are used to represent numbers with a fractional component.
    */
    let x: f32 = 2.0; // f32
    let y: f64 = 3.0; // f64
    let z = 2.0; // f64 default type

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);
    println!("---------------------------------------------");

    // Boolean type
    // Booleans represent logical values. Rust has a bool type that can be either true or false.
    let m: bool = true; // implicit declaration
    let n: bool = false; // explicit declaration
    println!("The value of m , n is: {}, {}", m, n);

    // if condition
    if m {
        println!("m is true");
    } else {
        println!("m is false");
    }

    let not_m = !m;
    println!("The value of not_m is: {}", not_m);

    let b: bool = true; // must initialize with a value inlined
    println!("---------------------------------------------");

    // Characters
    // Rust's char type represents a single Unicode character. Characters are enclosed in single quotes.
    let o: char = 'a';
    let p: char = '😀';
    let q: char = 'A';
    println!("The value of o, p, q is: {}, {}, {}", o, p, q);
    println!("---------------------------------------------");

    // Tuples
    // A tuple is a collection of values of different types. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, 'a');

    // Destructuring
    let (a, b, c, d) = tup;
    println!("The value of a, b, c, d is: {}, {}, {}, {}", a, b, c, d);
    println!("---------------------------------------------");

    // Accessing tuple elements using index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
    println!("---------------------------------------------");

    // Arrays
    // An array is a collection of elements of the same type.
    // Arrays in Rust have a fixed length, like tuples.
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The value of first and second is: {}, {}", first, second);

    for element in arr.iter() {
        println!("The value of element is: {}", element);
    }
    println!("---------------------------------------------");
}
