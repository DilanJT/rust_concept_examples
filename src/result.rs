

fn main() {
    // Option to Result:
    let opt: Option<i32> = Some(42);
    let result = opt.ok_or("No value found");

    // Result to Option:
    let result: Result<i32, &str> = Err("Error occurred");
    let opt= result.ok();
    println!("Option from Result: {:?}", opt);

    
}