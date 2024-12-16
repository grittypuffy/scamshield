mod scraper;
use std::process::{Command, Child};
use fantoccini::ClientBuilder;
use fantoccini::wd::Capabilities;

/// Starts the Chromedriver process and returns the child process handle.
///
/// # Returns
/// * `Result<Child, std::io::Error>` - A handle to the running Chromedriver process.
fn start_chromedriver() -> Result<Child, std::io::Error> {
    println!("Starting Chromedriver...");
    Command::new("chromedriver")
        .arg("--port=4444") // Specify the port
        .spawn() // Spawns the process
}

/// Stops the Chromedriver process.
///
/// # Arguments
/// * `child` - The handle to the Chromedriver process.
///
/// Ensures that the process is terminated after scraping is complete.
fn stop_chromedriver(child: &mut Child) {
    println!("Stopping Chromedriver...");
    if let Err(e) = child.kill() {
        eprintln!("Failed to stop Chromedriver: {:?}", e);
    }
}

/// Creates browser capabilities for a headless Chrome session.
///
/// # Returns
/// * `Capabilities` - WebDriver capabilities for headless Chrome.
fn create_headless_capabilities() -> Capabilities {
    let mut caps = Capabilities::new();
    let chrome_opts = serde_json::json!({
        "args": ["--headless", "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"]
    });
    caps.insert("goog:chromeOptions".to_string(), chrome_opts);
    caps
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start Chromedriver
    let mut chromedriver = start_chromedriver()
        .expect("Failed to start Chromedriver. Ensure it is installed and in your PATH.");

    // Allow Chromedriver time to initialize
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Create headless Chrome capabilities
    let caps = create_headless_capabilities();

    // Connect to WebDriver with headless capabilities
    let fantoccini_client = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:4444")
        .await?;
    
    // Stop WebDriver client
    fantoccini_client.close().await?;

    // Stop Chromedriver
    stop_chromedriver(&mut chromedriver);
    
    Ok(())
}
