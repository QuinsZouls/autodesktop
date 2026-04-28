use enigo::{Enigo, Mouse, Keyboard, Coordinate, Direction, Key, Axis, Button};

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

    /// Move mouse to absolute position
    /// 
    /// WORKAROUND for macOS timing issue (https://github.com/enigo-rs/enigo/issues/182):
    /// Calling location() after move_mouse ensures the event is processed correctly.
    pub fn move_to(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.enigo
            .move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| format!("Failed to move mouse: {}", e))?;
        
        // macOS workaround: force event processing by querying location
        #[cfg(target_os = "macos")]
        {
            let _ = self.enigo.location();
        }
        
        Ok(())
    }

    /// Move mouse relative to current position
    /// 
    /// WORKAROUND for macOS timing issue (https://github.com/enigo-rs/enigo/issues/182):
    /// Calling location() after move_mouse ensures the event is processed correctly.
    pub fn move_relative(&mut self, dx: i32, dy: i32) -> Result<(), String> {
        self.enigo
            .move_mouse(dx, dy, Coordinate::Rel)
            .map_err(|e| format!("Failed to move mouse: {}", e))?;
        
        // macOS workaround: force event processing by querying location
        #[cfg(target_os = "macos")]
        {
            let _ = self.enigo.location();
        }
        
        Ok(())
    }

    /// Click mouse button at current position
    pub fn click(&mut self, button: MouseButton) -> Result<(), String> {
        self.enigo
            .button(button.into(), Direction::Click)
            .map_err(|e| format!("Failed to click: {}", e))
    }

    /// Click at specific coordinates
    pub fn click_at(&mut self, x: i32, y: i32, button: MouseButton) -> Result<(), String> {
        self.move_to(x, y)?;
        self.click(button)
    }

    /// Double click at specific coordinates
    pub fn double_click_at(&mut self, x: i32, y: i32, button: MouseButton) -> Result<(), String> {
        self.move_to(x, y)?;
        self.enigo
            .button(button.into(), Direction::Click)
            .map_err(|e| format!("Failed to click: {}", e))?;
        self.enigo
            .button(button.into(), Direction::Click)
            .map_err(|e| format!("Failed to click: {}", e))
    }

    /// Drag from one point to another
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

    /// Scroll the mouse wheel (vertical)
    pub fn scroll_vertical(&mut self, amount: i32) -> Result<(), String> {
        // Positive = scroll up, negative = scroll down
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

    /// Type a string of text
    pub fn type_text(&mut self, text: &str) -> Result<(), String> {
        self.enigo
            .text(text)
            .map_err(|e| format!("Failed to type text: {}", e))
    }

    /// Press and release a single key
    pub fn key(&mut self, key: Key) -> Result<(), String> {
        self.enigo
            .key(key, Direction::Click)
            .map_err(|e| format!("Failed to press key: {}", e))
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
