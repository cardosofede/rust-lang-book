use core::num;

fn main() {
    let mut x: i32 = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const SUBSCRIBER_COUNT: u32 = 100000;

    // Integers
    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)
    let f: u8 = 254;

    // Floating-point numbers
    let f: f64 = 2.0;
    let g: f32 = 3.0;

    // Addition
    let sum: i32 = 5 + 10;
    
    // Substraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let division = 56.7 / 32.2;

    // Remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false;

    // Character
    let c = 'z';
    let rocket = '🚀';

    // Tuples
    let tup = ("Dardonacci", 100_000);
    let (channel, sub_count) = tup;
    let sub_account = tup.1;

    // Arrays
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    let byte = [32; 8];

    my_function(4, byte);
    compare_number(45, 48);
    just_loop(10);
    while_loop();
    for_loop();
}


fn my_function(x: i32, y: [i32; 8]) {
    println!("Another function.");
    println!("X: {}", x);
    println!("Y: {:?}", y)
    }

fn compare_number(x: i32, y: i32) {
    if x > y {
        println!("X is bigger than Y")
    } else if x < y {
        println!("Y is bigger than X")
    } else {
        println!("Both numbers are equal")
    }
}

fn just_loop(x: i32) {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == x {
            break counter;
        }
    };
    println!("The result of the loop is {}", result)
}

fn while_loop() {
    let mut number = 6;
    
    while number != 0 {
        println!("Number {}", number);
        number -= 1;
    }
    println!("Loop finished")
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is {}", element)
    }

    for number in 1..4 {
        println!("Number {}!", number)
    }
}
