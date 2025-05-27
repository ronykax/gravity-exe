use std::{thread, time::Duration};
use windows::{Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            println!("No active window");
            return;
        }

        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &mut rect).is_err() {
            println!("Failed to get window rect");
            return;
        }

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        // Get screen height
        let hdc = GetDC(hwnd);
        let screen_height = GetDeviceCaps(hdc, VERTRES);
        ReleaseDC(hwnd, hdc);

        let gravity = 1.1; // acceleration factor

        // Main loop: keep checking and falling if needed
        loop {
            // Get current window position in case user moved it
            if GetWindowRect(hwnd, &mut rect).is_err() {
                println!("Failed to get window rect");
                break;
            }
            let mut y = rect.top;
            let mut velocity = 1.0;

            // Only fall if not already at or below the bottom
            if y + height < screen_height {
                // Simulate falling
                while y + height < screen_height {
                    y += velocity as i32;
                    velocity *= gravity;

                    if y + height >= screen_height {
                        y = screen_height - height;
                    }

                    MoveWindow(hwnd, rect.left, y, width, height, true).unwrap();
                    thread::sleep(Duration::from_millis(16)); // ~60fps
                }
            }

            // Sleep a bit before checking again if the window was moved up
            thread::sleep(Duration::from_millis(100));
        }
    }
}
