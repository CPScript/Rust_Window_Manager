```
cargo build
ssh -X username@host "cd <Path-to>/Rust_Window_Manager && cargo run"
```


can be improved via adding:

```
1. Expanding Task Management: Add features for managing tasks, like killing processes or prioritizing them.
2. Improving Network Features: Add additional networking capabilities, such as posting data or handling websockets.
3. User Interface: Make a more user-friendly interface for managing tasks and networking directly from the window manager.
4. Configuration Management: Add a configuration system to allow users to customize settings.
```

---

### Features
* **Window Management**: Create and manage a simple X11 window.
* **Task Management**: List current processes running on the system.
* **Network Management**: Fetch data from a specified URL asynchronously.

## Get started
* **Rust**: HAVE RIST INSTALLED... you have Rust installed.
* **X11 Libraries**: Make sure to install the necessary development libraries for X11:
```
sudo apt install libx11-dev
```
* **Network Access**: Ensure that you have network access to perform HTTP requests.

### Clone the Repository
```
git clone https://github.com/CPScript/Rust_Window_Manager.git
cd Rust_Window_Manager
```

### Build
* **Compile** the repo using Cargo:
```
cargo build
```

### Execute the repo
* **To run the repo** with X11 forwarding, use the following command (ensure you are connected via SSH with X11 forwarding enabled):
```
ssh -X username@host "cd <Path-to>/Rust_Window_Manager && cargo run"
```

### Output
* **When the repo runs**, it will display a window and print the current tasks in the terminal, as well as fetch data from a specified URL.

---

## Structure

```
simple_window_manager/
├── Cargo.toml            # Project configuration and dependencies
└── src                   # Source code
    ├── main.rs           # Entry point for the application
    ├── lib.rs            # Library file declaring modules
    ├── window             # Module for window management
    │   ├── mod.rs        # Module declaration
    │   └── manager.rs    # WindowManager implementation
    ├── task              # Module for task management
    │   ├── mod.rs        # Module declaration
    │   └── manager.rs    # TaskManager implementation
    └── network           # Module for network operations
        ├── mod.rs        # Module declaration
        └── manager.rs    # NetworkManager implementation
```

