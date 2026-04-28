use enigo::{Enigo, Mouse, Keyboard, Coordinate, Direction, Key, Axis, Button};
use std::thread;
use std::time::{Duration, Instant};

/// Mouse control wrapper
pub struct MouseController {
    enigo: Enigo,
}

impl MouseController {
    pub fn new() -> Result<Self, String> {
        let enigo = Enigo::new(&Default::default())
            .map_err(|e| format!("Failed to initialize enigo: {}", e))?;
        Ok(Self { enigo })
    }

    /// Move mouse to absolute position (instant)
    pub fn move_to(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.enigo
            .move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| format!("Failed to move mouse: {}", e))?;
        
        #[cfg(target_os = "macos")]
        { let _ = self.enigo.location(); }
        
        Ok(())
    }

    /// Move mouse relative to current position
    pub fn move_relative(&mut self, dx: i32, dy: i32) -> Result<(), String> {
        self.enigo
            .move_mouse(dx, dy, Coordinate::Rel)
            .map_err(|e| format!("Failed to move mouse: {}", e))?;
        
        #[cfg(target_os = "macos")]
        { let _ = self.enigo.location(); }
        
        Ok(())
    }

    /// Move mouse smoothly to position with natural-looking motion
    /// Uses quadratic Bezier curve with random deviation for human-like movement
    pub fn move_to_smooth(&mut self, x: i32, y: i32, duration_ms: u32) -> Result<(), String> {
        let (start_x, start_y) = self.get_position()?;
        
        // Calculate distance
        let dx = x - start_x;
        let dy = y - start_y;
        let distance = ((dx * dx + dy * dy) as f64).sqrt();
        
        // Adjust duration based on distance (min 100ms, max 2s)
        let actual_duration = duration_ms.max(100).min(2000);
        let steps = (distance / 10.0).max(10.0).min(100.0) as usize;
        
        // Generate random control point for Bezier curve (adds human-like deviation)
        let mid_x = start_x + dx / 2;
        let mid_y = start_y + dy / 2;
        
        // Random deviation perpendicular to the path
        let deviation = (distance * 0.1).min(50.0);
        let angle = (dy as f64).atan2(dx as f64) + std::f64::consts::FRAC_PI_2;
        let ctrl_x = mid_x + (deviation * angle.cos()) as i32;
        let ctrl_y = mid_y + (deviation * angle.sin()) as i32;
        
        let step_duration = Duration::from_micros((actual_duration as u64 * 1000) / steps as u64);
        
        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            
            // Quadratic Bezier: B(t) = (1-t)²P0 + 2(1-t)tP1 + t²P2
            let px = ((1.0 - t).powi(2) * start_x as f64 
                     + 2.0 * (1.0 - t) * t * ctrl_x as f64 
                     + t.powi(2) * x as f64) as i32;
            let py = ((1.0 - t).powi(2) * start_y as f64 
                     + 2.0 * (1.0 - t) * t * ctrl_y as f64 
                     + t.powi(2) * y as f64) as i32;
            
            self.move_to(px, py)?;
            thread::sleep(step_duration);
        }
        
        // Ensure we end at exact position
        self.move_to(x, y)?;
        
        Ok(())
    }

    /// Move smoothly with default duration
    pub fn move_to_human(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.move_to_smooth(x, y, 500)
    }

    /// Get current mouse position
    fn get_position(&mut self) -> Result<(i32, i32), String> {
        #[cfg(target_os = "macos")]
        {
            self.enigo.location()
                .map(|(x, y)| (x, y))
                .map_err(|e| format!("Failed to get position: {}", e))
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            // Fallback: use get_mouse_position() for other platforms
            get_mouse_position()
        }
    }

    /// Click mouse button at current position
    pub fn click(&mut self, button: MouseButton) -> Result<(), String> {
        self.enigo
            .button(button.into(), Direction::Click)
            .map_err(|e| format!("Failed to click: {}", e))
    }

    /// Click at specific coordinates (instant)
    pub fn click_at(&mut self, x: i32, y: i32, button: MouseButton) -> Result<(), String> {
        self.move_to(x, y)?;
        self.click(button)
    }

    /// Click at specific coordinates with smooth movement
    pub fn click_at_smooth(&mut self, x: i32, y: i32, button: MouseButton) -> Result<(), String> {
        self.move_to_human(x, y)?;
        // Small delay before click (human-like)
        thread::sleep(Duration::from_millis(50 + rand_delay(30)));
        self.click(button)
    }

    /// Double click at specific coordinates
    pub fn double_click_at(&mut self, x: i32, y: i32, button: MouseButton) -> Result<(), String> {
        self.move_to(x, y)?;
        self.enigo
            .button(button.into(), Direction::Click)
            .map_err(|e| format!("Failed to click: {}", e))?;
        // Human-like delay between clicks
        thread::sleep(Duration::from_millis(100 + rand_delay(50)));
        self.enigo
            .button(button.into(), Direction::Click)
            .map_err(|e| format!("Failed to click: {}", e))
    }

    /// Drag from one point to another (instant)
    pub fn drag(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, button: MouseButton) -> Result<(), String> {
        self.move_to(x1, y1)?;
        self.enigo
            .button(button.into(), Direction::Press)
            .map_err(|e| format!("Failed to press button: {}", e))?;
        self.move_to(x2, y2)?;
        self.enigo
            .button(button.into(), Direction::Release)
            .map_err(|e| format!("Failed to release button: {}", e))
    }

    /// Drag from one point to another with smooth movement
    pub fn drag_smooth(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, button: MouseButton) -> Result<(), String> {
        self.move_to_human(x1, y1)?;
        thread::sleep(Duration::from_millis(50));
        self.enigo
            .button(button.into(), Direction::Press)
            .map_err(|e| format!("Failed to press button: {}", e))?;
        self.move_to_smooth(x2, y2, 300)?;
        self.enigo
            .button(button.into(), Direction::Release)
            .map_err(|e| format!("Failed to release button: {}", e))
    }

    /// Scroll the mouse wheel (vertical)
    pub fn scroll_vertical(&mut self, amount: i32) -> Result<(), String> {
        self.enigo
            .scroll(amount, Axis::Vertical)
            .map_err(|e| format!("Failed to scroll: {}", e))
    }

    /// Scroll the mouse wheel (horizontal)
    pub fn scroll_horizontal(&mut self, amount: i32) -> Result<(), String> {
        self.enigo
            .scroll(amount, Axis::Horizontal)
            .map_err(|e| format!("Failed to scroll: {}", e))
    }

    /// Scroll both axes
    pub fn scroll(&mut self, x: i32, y: i32) -> Result<(), String> {
        if x != 0 {
            self.scroll_horizontal(x)?;
        }
        if y != 0 {
            self.scroll_vertical(y)?;
        }
        Ok(())
    }
}

/// Mouse buttons
#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl From<MouseButton> for Button {
    fn from(btn: MouseButton) -> Self {
        match btn {
            MouseButton::Left => Button::Left,
            MouseButton::Right => Button::Right,
            MouseButton::Middle => Button::Middle,
        }
    }
}

/// Keyboard control wrapper
pub struct KeyboardController {
    enigo: Enigo,
}

impl KeyboardController {
    pub fn new() -> Result<Self, String> {
        let enigo = Enigo::new(&Default::default())
            .map_err(|e| format!("Failed to initialize enigo: {}", e))?;
        Ok(Self { enigo })
    }

    /// Type text instantly
    pub fn type_text(&mut self, text: &str) -> Result<(), String> {
        self.enigo
            .text(text)
            .map_err(|e| format!("Failed to type text: {}", e))
    }

    /// Type text with human-like timing and variation
    /// - Random delay between keystrokes (40-120ms base)
    /// - Occasional longer pauses (like thinking)
    /// - Small speed variation per character
    pub fn type_human(&mut self, text: &str) -> Result<(), String> {
        for ch in text.chars() {
            // Base delay with random variation
            let base_delay = 60u64 + rand_delay(40);
            
            // Occasional "thinking" pause (5% chance)
            let thinking_pause = if rand_chance(0.05) {
                150u64 + rand_delay(100)
            } else {
                0
            };
            
            // Type the character
            self.enigo
                .text(&ch.to_string())
                .map_err(|e| format!("Failed to type character '{}': {}", ch, e))?;
            
            // Wait before next character
            thread::sleep(Duration::from_millis(base_delay + thinking_pause));
        }
        
        Ok(())
    }

    /// Type text with configurable speed
    /// speed: 1-10 (1=slowest, 10=fastest)
    pub fn type_at_speed(&mut self, text: &str, speed: u8) -> Result<(), String> {
        let speed = speed.max(1).min(10);
        let base_delay = 120u64 - (speed as u64 * 10); // 110ms at speed 1, 20ms at speed 10
        
        for ch in text.chars() {
            let delay = base_delay + rand_delay(20);
            self.enigo
                .text(&ch.to_string())
                .map_err(|e| format!("Failed to type character '{}': {}", ch, e))?;
            thread::sleep(Duration::from_millis(delay));
        }
        
        Ok(())
    }

    /// Press and release a single key
    pub fn key(&mut self, key: Key) -> Result<(), String> {
        self.enigo
            .key(key, Direction::Click)
            .map_err(|e| format!("Failed to press key: {}", e))
    }

    /// Press key with human-like timing
    pub fn key_human(&mut self, key: Key) -> Result<(), String> {
        // Small delay before pressing
        thread::sleep(Duration::from_millis(30 + rand_delay(20)));
        
        self.enigo
            .key(key, Direction::Press)
            .map_err(|e| format!("Failed to press key: {}", e))?;
        
        // Hold time varies (50-150ms)
        thread::sleep(Duration::from_millis(50 + rand_delay(100)));
        
        self.enigo
            .key(key, Direction::Release)
            .map_err(|e| format!("Failed to release key: {}", e))
    }

    /// Press a key combination (e.g., Ctrl+C)
    pub fn combo(&mut self, modifiers: &[Key], key: Key) -> Result<(), String> {
        // Press all modifiers
        for modifier in modifiers {
            self.enigo
                .key(*modifier, Direction::Press)
                .map_err(|e| format!("Failed to press modifier: {}", e))?;
        }

        // Press and release the main key
        self.enigo
            .key(key, Direction::Click)
            .map_err(|e| format!("Failed to press key: {}", e))?;

        // Release all modifiers
        for modifier in modifiers {
            self.enigo
                .key(*modifier, Direction::Release)
                .map_err(|e| format!("Failed to release modifier: {}", e))?;
        }

        Ok(())
    }

    /// Press combo with human-like timing
    pub fn combo_human(&mut self, modifiers: &[Key], key: Key) -> Result<(), String> {
        // Small delay before starting combo
        thread::sleep(Duration::from_millis(50 + rand_delay(30)));
        
        // Press modifiers with small delays between each
        for modifier in modifiers {
            thread::sleep(Duration::from_millis(20 + rand_delay(10)));
            self.enigo
                .key(*modifier, Direction::Press)
                .map_err(|e| format!("Failed to press modifier: {}", e))?;
        }

        // Press main key
        thread::sleep(Duration::from_millis(30 + rand_delay(20)));
        self.enigo
            .key(key, Direction::Click)
            .map_err(|e| format!("Failed to press key: {}", e))?;

        // Release modifiers
        for modifier in modifiers {
            thread::sleep(Duration::from_millis(15 + rand_delay(10)));
            self.enigo
                .key(*modifier, Direction::Release)
                .map_err(|e| format!("Failed to release modifier: {}", e))?;
        }

        Ok(())
    }

    /// Hold a key down
    pub fn press(&mut self, key: Key) -> Result<(), String> {
        self.enigo
            .key(key, Direction::Press)
            .map_err(|e| format!("Failed to press key: {}", e))
    }

    /// Release a key
    pub fn release(&mut self, key: Key) -> Result<(), String> {
        self.enigo
            .key(key, Direction::Release)
            .map_err(|e| format!("Failed to release key: {}", e))
    }
}

/// Generate random delay in range 0-max_ms
/// Uses simple pseudo-random based on system time
fn rand_delay(max_ms: u64) -> u64 {
    let now = Instant::now();
    let nanos = now.elapsed().as_nanos();
    (nanos % ((max_ms + 1) as u128)) as u64
}

/// Check if random chance occurs (probability 0.0-1.0)
fn rand_chance(probability: f64) -> bool {
    let now = Instant::now();
    let nanos = now.elapsed().as_nanos();
    let rand_val = (nanos % 1000) as f64 / 1000.0;
    rand_val < probability
}

/// Parse a key name string to enigo::Key
pub fn parse_key(key_str: &str) -> Result<Key, String> {
    match key_str.to_lowercase().as_str() {
        "enter" | "return" => Ok(Key::Return),
        "esc" | "escape" => Ok(Key::Escape),
        "tab" => Ok(Key::Tab),
        "backspace" | "delete" => Ok(Key::Backspace),
        "space" | " " => Ok(Key::Space),
        "shift" => Ok(Key::Shift),
        "ctrl" | "control" => Ok(Key::Control),
        "alt" | "option" => Ok(Key::Alt),
        "cmd" | "super" | "meta" | "command" => Ok(Key::Meta),
        "up" | "arrow_up" => Ok(Key::UpArrow),
        "down" | "arrow_down" => Ok(Key::DownArrow),
        "left" | "arrow_left" => Ok(Key::LeftArrow),
        "right" | "arrow_right" => Ok(Key::RightArrow),
        "home" => Ok(Key::Home),
        "end" => Ok(Key::End),
        "pageup" | "page_up" => Ok(Key::PageUp),
        "pagedown" | "page_down" => Ok(Key::PageDown),
        "delete_key" | "forward_delete" => Ok(Key::Delete),
        "f1" => Ok(Key::F1),
        "f2" => Ok(Key::F2),
        "f3" => Ok(Key::F3),
        "f4" => Ok(Key::F4),
        "f5" => Ok(Key::F5),
        "f6" => Ok(Key::F6),
        "f7" => Ok(Key::F7),
        "f8" => Ok(Key::F8),
        "f9" => Ok(Key::F9),
        "f10" => Ok(Key::F10),
        "f11" => Ok(Key::F11),
        "f12" => Ok(Key::F12),
        // For punctuation and symbols, use Unicode
        s if s.len() == 1 => {
            let ch = s.chars().next().unwrap();
            Ok(Key::Unicode(ch))
        }
        _ => Err(format!("Unknown key: '{}'", key_str)),
    }
}

/// Parse modifier keys from a string like "ctrl+shift" or "cmd+alt"
pub fn parse_modifiers(mod_str: &str) -> Result<Vec<Key>, String> {
    mod_str
        .split('+')
        .map(|s| parse_key(s.trim()))
        .collect()
}

/// Parse a combo string like "Ctrl+C" or "Alt+Tab"
pub fn parse_combo(combo_str: &str) -> Result<(Vec<Key>, Key), String> {
    let parts: Vec<&str> = combo_str.split('+').collect();
    if parts.len() < 2 {
        return Err(format!("Combo must have at least 2 keys, e.g., 'Ctrl+C'. Got: '{}'", combo_str));
    }

    let mut modifiers = Vec::new();
    for part in &parts[..parts.len() - 1] {
        modifiers.push(parse_key(part.trim())?);
    }

    let key = parse_key(parts[parts.len() - 1].trim())?;

    Ok((modifiers, key))
}

/// Get current mouse position (macOS only)
pub fn get_mouse_position() -> Result<(i32, i32), String> {
    #[cfg(target_os = "macos")]
    {
        use core_graphics::event::CGEvent;
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
        
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
            .map_err(|_| "Failed to create CGEventSource".to_string())?;
        
        let event = CGEvent::new(source)
            .map_err(|_| "Failed to create CGEvent".to_string())?;
        
        let location = event.location();
        Ok((location.x as i32, location.y as i32))
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Err("Getting mouse position is only supported on macOS".to_string())
    }
}