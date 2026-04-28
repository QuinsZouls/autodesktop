mod commands;
mod capture;
mod input;
mod overlay;

use clap::Parser;
use commands::{Cli, Commands, MouseCommands, KeyboardCommands};
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Capture(args) => cmd_capture(&args),
        Commands::Mouse(mouse_cmd) => cmd_mouse(&mouse_cmd),
        Commands::Keyboard(key_cmd) => cmd_keyboard(&key_cmd),
        Commands::Info => cmd_info(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Handle capture command
fn cmd_capture(args: &commands::CaptureArgs) -> Result<(), String> {
    let region = args.parse_region()?;
    let grid_config = args.grid_config();
    let show_mouse = args.show_mouse && !args.no_grid; // Only show mouse if grid is enabled

    // Get mouse position before capture
    let mouse_pos = input::get_mouse_position();

    println!("Capturing screenshot...");

    let image = capture::capture_with_grid(
        args.display,
        region,
        grid_config,
        show_mouse,
    )?;

    let width = image.width();
    let height = image.height();

    // Save the image
    let output_path = Path::new(&args.output);
    image.save(output_path).map_err(|e| {
        format!("Failed to save image to '{}': {}", args.output, e)
    })?;

    let file_size = output_path.metadata().map(|m| m.len()).unwrap_or(0);
    let file_size_kb = file_size as f64 / 1024.0;

    println!("Screenshot saved to: {}", args.output);
    println!("  Resolution: {}x{} px", width, height);
    println!("  File size: {:.1} KB", file_size_kb);

    // Show mouse position
    if let Ok((mx, my)) = mouse_pos {
        println!("  Mouse position: ({}, {})", mx, my);
    }

    if !args.no_grid {
        let spacing = args.grid;
        let cols = width / spacing;
        let rows = height / spacing;
        println!("  Grid: {}px spacing ({} cols x {} rows)", spacing, cols, rows);
        if args.grid_labels && !args.no_grid_labels {
            println!("  Labels: enabled (size: {:.0}px)", args.label_size);
        } else {
            println!("  Labels: disabled");
        }
    }
    
    if show_mouse {
        println!("  Mouse indicator: enabled");
    }

    if let Some((x, y, w, h)) = args.parse_region()? {
        println!("  Region: ({}, {}, {}, {})", x, y, w, h);
    }

    Ok(())
}

/// Handle mouse commands
fn cmd_mouse(cmd: &MouseCommands) -> Result<(), String> {
    let mut mouse = input::MouseController::new()?;

    match cmd {
        MouseCommands::Move { x, y, instant, duration } => {
            if *instant {
                mouse.move_to(*x, *y)?;
                println!("Mouse moved to ({}, {}) [instant]", x, y);
            } else {
                mouse.move_to_smooth(*x, *y, *duration)?;
                println!("Mouse moved smoothly to ({}, {}) in {}ms", x, y, duration);
            }
        }
        MouseCommands::Click { button } => {
            let btn = commands::parse_mouse_button(button)?;
            mouse.click(btn)?;
            println!("{} click at current position", button);
        }
        MouseCommands::ClickAt { x, y, button, instant } => {
            let btn = commands::parse_mouse_button(button)?;
            if *instant {
                mouse.click_at(*x, *y, btn)?;
                println!("{} click at ({}, {}) [instant]", button, x, y);
            } else {
                mouse.click_at_smooth(*x, *y, btn)?;
                println!("{} click at ({}, {}) [smooth]", button, x, y);
            }
        }
        MouseCommands::DoubleClick { x, y, button } => {
            let btn = commands::parse_mouse_button(button)?;
            mouse.double_click_at(*x, *y, btn)?;
            println!("Double {} click at ({}, {})", button, x, y);
        }
        MouseCommands::Drag { x1, y1, x2, y2, button, instant } => {
            let btn = commands::parse_mouse_button(button)?;
            if *instant {
                mouse.drag(*x1, *y1, *x2, *y2, btn)?;
                println!("Drag from ({}, {}) to ({}, {}) [instant]", x1, y1, x2, y2);
            } else {
                mouse.drag_smooth(*x1, *y1, *x2, *y2, btn)?;
                println!("Drag from ({}, {}) to ({}, {}) [smooth]", x1, y1, x2, y2);
            }
        }
        MouseCommands::Scroll { x, y } => {
            mouse.scroll(*x, *y)?;
            println!("Scrolled (horizontal: {}, vertical: {})", x, y);
        }
    }

    Ok(())
}

/// Handle keyboard commands
fn cmd_keyboard(cmd: &KeyboardCommands) -> Result<(), String> {
    let mut keyboard = input::KeyboardController::new()?;

    match cmd {
        KeyboardCommands::TypeText { text, instant, speed } => {
            if *instant {
                keyboard.type_text(text)?;
                println!("Typed: {} [instant]", text);
            } else {
                keyboard.type_at_speed(text, *speed)?;
                println!("Typed (human speed {}): {}", speed, text);
            }
        }
        KeyboardCommands::Key { key, instant } => {
            let key_enum = input::parse_key(key)?;
            if *instant {
                keyboard.key(key_enum)?;
                println!("Pressed key: {} [instant]", key);
            } else {
                keyboard.key_human(key_enum)?;
                println!("Pressed key: {} [human]", key);
            }
        }
        KeyboardCommands::Combo { combo, instant } => {
            let (modifiers, key) = input::parse_combo(combo)?;
            if *instant {
                keyboard.combo(&modifiers, key)?;
                println!("Combo: {} [instant]", combo);
            } else {
                keyboard.combo_human(&modifiers, key)?;
                println!("Combo: {} [human]", combo);
            }
        }
    }

    Ok(())
}

/// Handle info command
fn cmd_info() -> Result<(), String> {
    println!("Available displays:");
    println!();

    let monitors = capture::list_monitors()?;

    for m in &monitors {
        let primary_marker = if m.is_primary { " [PRIMARY]" } else { "" };
        println!("  [{}] {}{} ({}x{})", m.index, m.name, primary_marker, m.width, m.height);
    }

    println!();
    println!("Tip: Use --display <index> to select a specific monitor");
    println!("Tip: Use --grid 100 to add a coordinate grid overlay");

    Ok(())
}
