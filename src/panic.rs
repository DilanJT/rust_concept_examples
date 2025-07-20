#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        panic!("Oops!");
    });
    
    // ✅ Always handle JoinHandle results
    match handle.await {
        Ok(result) => println!("Task completed: {:?}", result),
        Err(e) => println!("Task panicked: {:?}", e),
    }

    println!("Main function completed");

    match handle_panic().await {
        Ok(_) => println!("Panic handled successfully"),
        Err(e) => println!("Caught panic: {:?}", e),
    }
}

async fn handle_panic() -> Result<(), Box<dyn std::error::Error>> {
    panic!("This is a panic in an async function");
    Ok(())
}