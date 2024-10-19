use x11::xlib::*;
use std::ptr;
use std::mem;
use std::ffi::CString;

struct WindowManager {
    display: *mut Display,
    window: Window,
    screen: i32,
}

impl WindowManager {
    fn new() -> Self {
        let display = unsafe { XOpenDisplay(ptr::null()) };
        if display.is_null() {
            panic!("Unable to open X display");
        }

        let screen = unsafe { XDefaultScreen(display) };
        let root_window = unsafe { XRootWindow(display, screen) };

        let window = unsafe {
            XCreateWindow(
                display,
                root_window,
                100,
                100,
                800,
                600,
                1,
                XBlackPixel(display, screen),
                XWhitePixel(display, screen),
            )
        };

        let title = CString::new("Rust Window Manager").unwrap();
        unsafe {
            XSetStandardProperties(display, window, title.as_ptr(), title.as_ptr(), 0, ptr::null(), 0, ptr::null());
            XSelectInput(display, window, ExposureMask | KeyPressMask | StructureNotifyMask);
            XMapWindow(display, window);
        }

        Self { display, window, screen }
    }

    fn run(&self) {
        loop {
            let mut event: XEvent = unsafe { mem::zeroed() };
            unsafe {
                XNextEvent(self.display, &mut event);
                match event.get_type() {
                    Expose => {
                        XFillRectangle(self.display, self.window, XBlackGC(self.display, self.screen), 20, 20, 100, 100);
                    }
                    KeyPress => {
                        println!("Exiting...");
                        break; 
                    }
                    ConfigureNotify => {
                        let configure_event: XConfigureEvent = mem::transmute(event);
                        println!("Window resized to {}x{}", configure_event.width, configure_event.height);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        unsafe {
            XDestroyWindow(self.display, self.window);
            XCloseDisplay(self.display);
        }
    }
}

fn main() {
    let manager = WindowManager::new();
    manager.run();
}
