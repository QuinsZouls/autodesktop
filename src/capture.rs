use xcap::Monitor;
use image::DynamicImage;
use crate::overlay::{GridConfig, apply_grid, draw_mouse_indicator};

/// Capture a screenshot of a specific monitor
pub fn capture_monitor(monitor_index: Option<usize>) -> Result<(DynamicImage, f32), String> {
    let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;

    if monitors.is_empty() {
        return Err("No monitors detected".to_string());
    }

    let monitor = match monitor_index {
        Some(idx) => monitors.iter().find(|m| {
            m.id().map(|id| id as usize == idx).unwrap_or(false)
        })
        .ok_or_else(|| {
            let available: Vec<_> = monitors.iter()
                .filter_map(|m| m.id().ok())
                .collect();
            format!("Monitor id {} not found. Available: {:?}", idx, available)
        })?,
        None => {
            // Use primary monitor
            monitors.iter().find(|m| {
                m.is_primary().unwrap_or(false)
            }).or_else(|| monitors.first())
            .ok_or("No primary monitor found")?
        }
    };

    // Get scale factor for converting logical points to pixels
    let scale_factor = monitor.scale_factor().map_err(|e| {
        format!("Failed to get scale factor: {}", e)
    })?;

    let rgba_image = monitor.capture_image().map_err(|e| {
        format!("Failed to capture monitor: {}", e)
    })?;

    let dynamic = DynamicImage::ImageRgba8(rgba_image);
    Ok((dynamic, scale_factor))
}

/// Capture a specific region of a monitor
pub fn capture_region(
    monitor_index: Option<usize>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<(DynamicImage, f32), String> {
    let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;
    if monitors.is_empty() {
        return Err("No monitors detected".to_string());
    }

    let monitor = match monitor_index {
        Some(idx) => monitors.iter().find(|m| {
            m.id().map(|id| id as usize == idx).unwrap_or(false)
        })
        .ok_or_else(|| format!("Monitor id {} not found", idx))?,
        None => {
            monitors.iter().find(|m| m.is_primary().unwrap_or(false))
                .or_else(|| monitors.first())
                .ok_or("No monitor found")?
        }
    };

    // Get scale factor
    let scale_factor = monitor.scale_factor().map_err(|e| {
        format!("Failed to get scale factor: {}", e)
    })?;

    let rgba_image = monitor.capture_region(x, y, width, height).map_err(|e| {
        format!("Failed to capture region: {}", e)
    })?;

    let dynamic = DynamicImage::ImageRgba8(rgba_image);
    Ok((dynamic, scale_factor))
}

/// Capture with optional grid overlay and mouse position indicator
pub fn capture_with_grid(
    monitor_index: Option<usize>,
    region: Option<(u32, u32, u32, u32)>,
    grid_config: Option<GridConfig>,
    show_mouse: bool,
) -> Result<DynamicImage, String> {
    let (image, scale_factor) = match region {
        Some((x, y, w, h)) => capture_region(monitor_index, x, y, w, h)?,
        None => capture_monitor(monitor_index)?,
    };

    // Apply grid with scale factor for correct logical coordinate labels
    let mut image = match grid_config {
        Some(mut config) => {
            config.scale_factor = scale_factor;
            apply_grid(image, &config)
        }
        None => image,
    };

    // Add mouse position indicator if requested
    if show_mouse {
        if let Ok((mx, my)) = crate::input::get_mouse_position() {
            // Convert logical points to pixels using scale factor
            let pixel_x = (mx as f32 * scale_factor) as i32;
            let pixel_y = (my as f32 * scale_factor) as i32;
            
            let indicator_color = image::Rgba([0, 255, 255, 200]); // Cyan
            let label_color = image::Rgba([255, 255, 255, 255]); // White
            image = draw_mouse_indicator(image, pixel_x, pixel_y, mx, my, indicator_color, label_color);
        }
    }

    Ok(image)
}

/// Get information about available monitors
pub fn list_monitors() -> Result<Vec<MonitorInfo>, String> {
    let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;

    let mut infos = Vec::new();
    for m in &monitors {
        let id = m.id().unwrap_or(0);
        let name = m.friendly_name().or_else(|_| m.name()).unwrap_or_else(|_| "Unknown".to_string());
        let width = m.width().unwrap_or(0);
        let height = m.height().unwrap_or(0);
        let is_primary = m.is_primary().unwrap_or(false);

        infos.push(MonitorInfo {
            index: id,
            name,
            width,
            height,
            is_primary,
        });
    }

    if infos.is_empty() {
        return Err("No monitors detected".to_string());
    }

    Ok(infos)
}

#[derive(Debug)]
pub struct MonitorInfo {
    pub index: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub is_primary: bool,
}
