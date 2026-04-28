use clap::{Parser, Subcommand, Args};
use crate::overlay::GridConfig;
use rgb::RGB8;

/// autodesktop - Screenshot capture with grid overlay and remote input control
#[derive(Parser, Debug)]
#[command(
    name = "autodesktop",
    version = "0.1.0",
    about = "CLI tool for screenshot capture with customizable grid overlay and remote input control (mouse/keyboard)",
    long_about = "autodesktop lets you capture screenshots with optional coordinate grids,\n\
                  and control mouse and keyboard input remotely.\n\n\
                  macOS requires Screen Recording and Accessibility permissions."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Capture a screenshot (optionally with grid overlay)
    Capture(CaptureArgs),

    /// Control mouse input
    #[command(subcommand)]
    Mouse(MouseCommands),

    /// Control keyboard input
    #[command(subcommand)]
    Keyboard(KeyboardCommands),

    /// List available displays and their resolutions
    Info,
}

/// Arguments for the capture command
#[derive(Args, Debug)]
pub struct CaptureArgs {
    /// Output file path (supports .png, .jpg, .bmp)
    #[arg(short, long, default_value = "screenshot.png")]
    pub output: String,

    /// Monitor index to capture (use 'info' command to list available)
    #[arg(short, long)]
    pub display: Option<usize>,

    /// Capture a specific region: X,Y,WIDTH,HEIGHT
    #[arg(long)]
    pub region: Option<String>,

    /// Grid spacing in pixels (default: 100, use --no-grid to disable)
    #[arg(long, default_value = "100")]
    pub grid: u32,

    /// Disable grid overlay
    #[arg(long, default_value = "false")]
    pub no_grid: bool,

    /// Grid line color: named (red, green, blue, yellow, white, cyan, orange), hex (#FF0000), or RGB (255,0,0)
    #[arg(long, default_value = "red")]
    pub grid_color: String,

    /// Grid line opacity (0.0 to 1.0)
    #[arg(long, default_value = "0.6")]
    pub grid_opacity: f32,

    /// Show coordinate labels along edges (default: true)
    #[arg(long, default_value = "true", action = clap::ArgAction::Set)]
    pub grid_labels: bool,

    /// Disable grid labels
    #[arg(long, default_value = "false")]
    pub no_grid_labels: bool,

    /// Font size for coordinate labels
    #[arg(long, default_value = "14")]
    pub label_size: f32,

    /// Show mouse position indicator on screenshot
    #[arg(long, default_value = "true")]
    pub show_mouse: bool,
}

impl CaptureArgs {
    /// Build GridConfig from arguments, returns None if grid is disabled
    pub fn grid_config(&self) -> Option<GridConfig> {
        if self.no_grid {
            return None;
        }
        
        let color = crate::overlay::parse_color(&self.grid_color)
            .unwrap_or(RGB8 { r: 255, g: 0, b: 0 });

        let show_labels = self.grid_labels && !self.no_grid_labels;

        Some(GridConfig {
            spacing: self.grid,
            color,
            opacity: self.grid_opacity.clamp(0.0, 1.0),
            show_labels,
            label_size: self.label_size.max(8.0).min(48.0),
        })
    }

    /// Parse region string "X,Y,W,H" into tuple
    pub fn parse_region(&self) -> Result<Option<(u32, u32, u32, u32)>, String> {
        match &self.region {
            Some(region_str) => {
                let parts: Vec<&str> = region_str.split(',').collect();
                if parts.len() != 4 {
                    return Err(format!(
                        "Region must be X,Y,WIDTH,HEIGHT (4 values). Got: '{}'",
                        region_str
                    ));
                }
                let x = parts[0].trim().parse::<u32>().map_err(|_| format!("Invalid X: '{}'", parts[0]))?;
                let y = parts[1].trim().parse::<u32>().map_err(|_| format!("Invalid Y: '{}'", parts[1]))?;
                let w = parts[2].trim().parse::<u32>().map_err(|_| format!("Invalid WIDTH: '{}'", parts[2]))?;
                let h = parts[3].trim().parse::<u32>().map_err(|_| format!("Invalid HEIGHT: '{}'", parts[3]))?;
                Ok(Some((x, y, w, h)))
            }
            None => Ok(None),
        }
    }
}

/// Mouse subcommands
#[derive(Subcommand, Debug)]
pub enum MouseCommands {
    /// Move cursor to absolute position
    Move {
        /// X coordinate
        x: i32,
        /// Y coordinate
        y: i32,
    },
    /// Click mouse button at current position
    Click {
        /// Mouse button (left, right, middle)
        #[arg(default_value = "left")]
        button: String,
    },
    /// Click at specific coordinates
    #[command(name = "click-at")]
    ClickAt {
        /// X coordinate
        x: i32,
        /// Y coordinate
        y: i32,
        /// Mouse button (left, right, middle)
        #[arg(default_value = "left")]
        button: String,
    },
    /// Double click at coordinates
    #[command(name = "double-click")]
    DoubleClick {
        /// X coordinate
        x: i32,
        /// Y coordinate
        y: i32,
        /// Mouse button (left, right, middle)
        #[arg(default_value = "left")]
        button: String,
    },
    /// Drag from one point to another
    Drag {
        /// Start X
        x1: i32,
        /// Start Y
        y1: i32,
        /// End X
        x2: i32,
        /// End Y
        y2: i32,
        /// Mouse button (left, right, middle)
        #[arg(default_value = "left")]
        button: String,
    },
    /// Scroll the mouse wheel
    Scroll {
        /// Horizontal scroll amount
        #[arg(default_value = "0")]
        x: i32,
        /// Vertical scroll amount (positive=up, negative=down)
        #[arg(default_value = "-3")]
        y: i32,
    },
}

/// Keyboard subcommands
#[derive(Subcommand, Debug)]
pub enum KeyboardCommands {
    /// Type text
    #[command(name = "type")]
    TypeText {
        /// Text to type
        text: String,
    },
    /// Press a single key
    Key {
        /// Key name (enter, esc, tab, space, ctrl, alt, cmd, up, down, left, right, f1-f12, etc.)
        key: String,
    },
    /// Press a key combination (e.g., "Ctrl+C", "Alt+Tab", "Cmd+Shift+4")
    Combo {
        /// Combination string (e.g., "Ctrl+C")
        combo: String,
    },
}

/// Parse mouse button string
pub fn parse_mouse_button(btn: &str) -> Result<crate::input::MouseButton, String> {
    match btn.to_lowercase().as_str() {
        "left" => Ok(crate::input::MouseButton::Left),
        "right" => Ok(crate::input::MouseButton::Right),
        "middle" | "center" => Ok(crate::input::MouseButton::Middle),
        _ => Err(format!("Unknown mouse button: '{}'. Use left, right, or middle.", btn)),
    }
}
