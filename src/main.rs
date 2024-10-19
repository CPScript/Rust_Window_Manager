use rust_window_manager::window::manager::WindowManager;
use rust_window_manager::task::manager::TaskManager;
use rust_window_manager::network::manager::NetworkManager;

#[tokio::main]
async fn main() {
    let manager = WindowManager::new(); // Initialize manager
    
    
    TaskManager::list_tasks(); // List current processes

    match NetworkManager::fetch_data("https://jsonplaceholder.typicode.com/posts/1").await { // Fetch from an example (example: https://jsonplaceholder.typicode.com/posts/1)
        Ok(data) => println!("Fetched Data: {}", data),
        Err(err) => eprintln!("Error fetching data: {}", err),
    }

    manager.run();
}
