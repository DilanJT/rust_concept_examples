

fn main() {
    let hello = get_hello("dilan");
    println!("{}", hello);

    // Usage:
    let mut my_string = String::from("hello");
    borrow_string(&my_string);        // ✅ my_string still valid
    modify_string(&mut my_string);    // ✅ my_string still valid
    take_ownership(my_string);        // ❌ my_string no longer valid after this

    let num_one = 10;
    let num_two = 20;
    let max = get_max(&num_one, &num_two);
    println!("Max is: {}", max);

    let mut s = String::from("hello");
    let r1 = &s;
    let r4 = &s;
    println!("{}, {}", r1, r4);
    let r2 = &mut s;
     // ❌ Error here
    println!("{}", r2);
    let r3 = &mut s;
    println!("{}", r3);

    let name = String::from("Alice");
    let length = get_length(name);
    println!("Length of name is: {}", length);




}

fn get_hello(name: &str) -> &str {
    let s = format!("Hello, {}", name);
    name
}

// Taking ownership (moves the value)
fn take_ownership(s: String) {
    println!("{}, and going out of scope", s);
} // s goes out of scope and is dropped

// Borrowing (doesn't take ownership)
fn borrow_string(s: &String) {
    println!("{}", s);
} // s goes out of scope, but since it's just a reference, nothing special happens

// Mutable borrowing
fn modify_string(s: &mut String) {
    s.push_str(" modified");
}

fn get_max<'n, 'b>(a: &'n i32, b: &'b i32) -> &'n i32 
    where 'b : 'n
{
    if a > b {
        a
    } else {
        b
    }
}

// Version A
fn process_string_a(s: String) -> String {
    format!("{} processed", s)
}

// Version B  
fn process_string_b(s: &str) -> String {
    format!("{} processed", s)
}

fn get_length(s: String) -> usize {
    s.len()
}
