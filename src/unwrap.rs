

fn main () {
    println!("Demonstrating unwrap_or and unwrap_or_else behavior:");
    demonstrate_when_called();
}

fn demonstrate_when_called() {
    println!("=== With Some(value) ===");
    
    // unwrap_or - default is computed even though not used
    let result1 = Some(42).unwrap_or({
        println!("unwrap_or: Computing default");
        0
    });

    println!("Result1: {}", result1);
    
    // unwrap_or_else - closure not called because we have Some
    let result2 = Some(42).unwrap_or_else(|| {
        println!("unwrap_or_else: Computing default");
        0
    });

    println!("Result2: {}", result2);
    
    println!("\n=== With None ===");
    
    // unwrap_or - default is computed and used
    let _result3 = None.unwrap_or({
        println!("unwrap_or: Computing default");
        0
    });

    println!("Result3: {}", _result3);
    
    // unwrap_or_else - closure is called and used
    let _result4: Option<i32> = None;
    let _result4 = None.unwrap_or_else(|| {
        println!("unwrap_or_else: Computing default");
        0
    });

    println!("Result4: {}", _result4);
}

// Output:
// === With Some(value) ===
// unwrap_or: Computing default        <- Always computed!
//                                     <- unwrap_or_else not called
// 
// === With None ===
// unwrap_or: Computing default        <- Computed and used
// unwrap_or_else: Computing default   <- Only now it's called