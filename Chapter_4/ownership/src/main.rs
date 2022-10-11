fn main() {
    // Ownership rules
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it's not yet declared
        let _s = "hello"; // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // Memory allocationss

    let x = 5;
    let y = x; // Copy

    println!("X is {}", x);
    println!("Y is {}", y);

    let s1 = String::from("Hello");
    let s2 = s1; // Move (not shallow copy)
    let s3 = s2.clone(); // This will keep s2 and s3
    println!("S2: {} | S3: {}", s2, s3);

    // Ownership

    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s); This won't work because now the ownership is 
    // inside the takes_ownership function and it's not returning the value

    let s = String::from("Hello");

    let s: String = takes_and_returns_ownership(s);
    println!("{}", s);

    // With variables that are in the stack, a copy will be created so we can use the variable in this scope.
    let x = 5;
    makes_copy(x);
    println!("{}", x);

    // The Rules of References
    // 1. At any given time, you can have either one mutable reference
    // or any number of inmutable references.
    // 2. References must always be valid.

    let s1 = String::from("Hello");
    let len = calculate_length(&s1); // this is called borrowing
    println!("The length of {} is {}.", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("{}", s2);

    // Slices
    // They are a reference of another array, and don't take ownership of that string.
    let mut s2 = String::from("Hello world!");
    let fw = first_word(&s2);
    println!("First word ends at index: {}", fw);

    let mut s = String::from("Hello world");
    let s2 = "hello world";

    let word = first_word_slice(&s);
    println!("{}", word);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_returns_ownership(some_string: String) -> String{
    some_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
