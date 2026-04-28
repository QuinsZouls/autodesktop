use image::{DynamicImage, Rgba};
use imageproc::drawing::{draw_cross_mut, draw_line_segment_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use rgb::RGB8;

/// Grid overlay configuration
#[derive(Clone, Debug)]
pub struct GridConfig {
    /// Spacing between grid lines in pixels
    pub spacing: u32,
    /// Grid line color (RGB)
    pub color: RGB8,
    /// Grid line opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Whether to show coordinate labels at intersections
    pub show_labels: bool,
    /// Font size for labels (default: 14)
    pub label_size: f32,
    /// Scale factor for converting pixels to logical coordinates (macOS Retina)
    pub scale_factor: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            spacing: 100,
            color: RGB8 {
                r: 255,
                g: 0,
                b: 0,
            },
            opacity: 0.6,
            show_labels: false,
            label_size: 14.0,
            scale_factor: 1.0,
        }
    }
}

impl GridConfig {
    /// Convert color + opacity to Rgba
    pub fn to_rgba(&self) -> Rgba<u8> {
        let alpha = (self.opacity * 255.0) as u8;
        Rgba([self.color.r, self.color.g, self.color.b, alpha])
    }

    /// Create an inverted color for better visibility on dark/light backgrounds
    pub fn to_rgba_inverted(&self) -> Rgba<u8> {
        let alpha = (self.opacity * 255.0) as u8;
        Rgba([
            255 - self.color.r,
            255 - self.color.g,
            255 - self.color.b,
            alpha,
        ])
    }
}

/// Parse a color string like "red", "yellow", "#FF0000", "255,0,0"
pub fn parse_color(color_str: &str) -> Result<RGB8, String> {
    match color_str.to_lowercase().as_str() {
        "red" | "rojo" => Ok(RGB8 { r: 255, g: 0, b: 0 }),
        "green" | "verde" | "lime" => Ok(RGB8 { r: 0, g: 255, b: 0 }),
        "blue" | "azul" => Ok(RGB8 { r: 0, g: 0, b: 255 }),
        "yellow" | "amarillo" => Ok(RGB8 { r: 255, g: 255, b: 0 }),
        "white" | "blanco" => Ok(RGB8 { r: 255, g: 255, b: 255 }),
        "black" | "negro" => Ok(RGB8 { r: 0, g: 0, b: 0 }),
        "cyan" => Ok(RGB8 { r: 0, g: 255, b: 255 }),
        "magenta" => Ok(RGB8 { r: 255, g: 0, b: 255 }),
        "orange" | "naranja" => Ok(RGB8 { r: 255, g: 165, b: 0 }),
        _ => {
            // Try hex format #RRGGBB
            if let Some(hex) = color_str.strip_prefix('#') {
                if hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..2], 16),
                        u8::from_str_radix(&hex[2..4], 16),
                        u8::from_str_radix(&hex[4..6], 16),
                    ) {
                        return Ok(RGB8 { r, g, b });
                    }
                }
            }
            // Try comma-separated format R,G,B
            let parts: Vec<&str> = color_str.split(',').collect();
            if parts.len() == 3 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    parts[0].trim().parse::<u8>(),
                    parts[1].trim().parse::<u8>(),
                    parts[2].trim().parse::<u8>(),
                ) {
                    return Ok(RGB8 { r, g, b });
                }
            }
            Err(format!("Invalid color format: '{}'. Use named color, #RRGGBB, or R,G,B", color_str))
        }
    }
}

/// Draw grid overlay on an image
pub fn apply_grid(mut image: DynamicImage, config: &GridConfig) -> DynamicImage {
    let width = image.width();
    let height = image.height();
    let rgba = config.to_rgba();
    let rgba_inv = config.to_rgba_inverted();

    // Convert to RgbaImage for drawing
    let mut img = image.to_rgba8();

    // Draw vertical lines
    let mut x = config.spacing;
    while x < width {
        draw_line_segment_mut(
            &mut img,
            (x as f32, 0.0),
            (x as f32, height as f32),
            rgba,
        );
        x += config.spacing;
    }

    // Draw horizontal lines
    let mut y = config.spacing;
    while y < height {
        draw_line_segment_mut(
            &mut img,
            (0.0, y as f32),
            (width as f32, y as f32),
            rgba,
        );
        y += config.spacing;
    }

    // Draw cross markers at intersections for better visibility
    x = config.spacing;
    while x < width {
        y = config.spacing;
        while y < height {
            draw_cross_mut(&mut img, rgba, x as i32, y as i32);
            y += config.spacing;
        }
        x += config.spacing;
    }

    image = DynamicImage::ImageRgba8(img);

    // Draw coordinate labels if enabled
    if config.show_labels {
        image = draw_coordinate_labels(image, config, &rgba_inv);
    }

    DynamicImage::ImageRgba8(image.to_rgba8())
}

/// Draw coordinate labels at grid edges (shows LOGICAL coordinates)
fn draw_coordinate_labels(
    mut image: DynamicImage,
    config: &GridConfig,
    label_color: &Rgba<u8>,
) -> DynamicImage {
    let width = image.width();
    let height = image.height();
    let scale = config.scale_factor;

    // Background color for label boxes
    let bg_color = Rgba([0, 0, 0, 180]);

    // Character cell dimensions (5x8 pixel bitmap font)
    let char_w = 6u32; // 5px char + 1px spacing
    let char_h = 9u32; // 8px char + 1px spacing

    // X-axis labels along the top edge (show logical coordinates)
    let mut pixel_x = config.spacing;
    while pixel_x < width {
        // Convert pixel to logical coordinate
        let logical_x = (pixel_x as f32 / scale) as i32;
        let label = format!("{}", logical_x);
        let label_w = (label.len() as u32 * char_w) + 4; // padding
        let label_h = char_h + 2;

        // Background box
        draw_filled_rect_mut(
            &mut image,
            Rect::at(pixel_x as i32 - (label_w as i32 / 2), 1)
                .of_size(label_w, label_h),
            bg_color,
        );

        // Draw digits
        let mut char_x = pixel_x as i32 - ((label.len() as i32 * char_w as i32) / 2) + 2;
        for ch in label.chars() {
            if ch.is_ascii_digit() {
                draw_digit(&mut image, char_x, 3, ch, *label_color);
            }
            char_x += char_w as i32;
        }

        pixel_x += config.spacing;
    }

    // Y-axis labels along the left edge (show logical coordinates)
    let mut pixel_y = config.spacing;
    while pixel_y < height {
        // Convert pixel to logical coordinate
        let logical_y = (pixel_y as f32 / scale) as i32;
        let label = format!("{}", logical_y);
        let label_w = (label.len() as u32 * char_w) + 4;
        let label_h = char_h + 2;

        // Background box
        draw_filled_rect_mut(
            &mut image,
            Rect::at(1, pixel_y as i32 - (label_h as i32 / 2))
                .of_size(label_w, label_h),
            bg_color,
        );

        // Draw digits
        let mut char_x = 3;
        for ch in label.chars() {
            if ch.is_ascii_digit() {
                draw_digit(&mut image, char_x, pixel_y as i32 - 3, ch, *label_color);
            }
            char_x += char_w as i32;
        }

        pixel_y += config.spacing;
    }

    image
}

/// Draw a single digit using 5x8 bitmap font
fn draw_digit(image: &mut DynamicImage, x: i32, y: i32, digit: char, color: Rgba<u8>) {
    if let Some(d) = digit.to_digit(10) {
        let pattern = DIGIT_PATTERNS[d as usize];
        for dy in 0u32..8 {
            for dx in 0u32..5 {
                let bit_index = dy * 5 + (4 - dx);
                if (pattern >> bit_index) & 1 == 1 {
                    draw_filled_rect_mut(
                        image,
                        Rect::at(x + dx as i32, y + dy as i32).of_size(1, 1),
                        color,
                    );
                }
            }
        }
    }
}

/// 5x8 bitmap patterns for digits 0-9
const DIGIT_PATTERNS: [u64; 10] = [
    0b01110_10001_10001_10001_10001_10001_10001_01110, // 0
    0b00100_01100_00100_00100_00100_00100_00100_01110, // 1
    0b01110_10001_00001_00010_00100_01000_10000_11111, // 2
    0b01110_10001_00001_00110_00001_00001_10001_01110, // 3
    0b00010_00110_01010_10010_11111_00010_00010_00010, // 4
    0b11111_10000_10000_11110_00001_00001_10001_01110, // 5
    0b01110_10001_10000_11110_10001_10001_10001_01110, // 6
    0b11111_00001_00010_00100_01000_01000_01000_01000, // 7
    0b01110_10001_10001_01110_10001_10001_10001_01110, // 8
    0b01110_10001_10001_10001_01111_00001_00001_01110, // 9
];

/// Draw a center crosshair for the current cursor position
pub fn draw_center_crosshair(mut image: DynamicImage, color: Rgba<u8>) -> DynamicImage {
    let width = image.width();
    let height = image.height();
    let cx = (width / 2) as i32;
    let cy = (height / 2) as i32;

    let mut img = image.to_rgba8();
    draw_cross_mut(&mut img, color, cx, cy);
    image = DynamicImage::ImageRgba8(img);

    // Draw center dot
    draw_filled_rect_mut(
        &mut image,
        Rect::at(cx - 2, cy - 2).of_size(5, 5),
        color,
    );

    image
}

/// Draw mouse position indicator at specific coordinates
/// 
/// Parameters:
/// - pixel_x, pixel_y: Position in pixels (for drawing on the image)
/// - logical_x, logical_y: Position in logical points (for display label - what user uses for mouse.move)
pub fn draw_mouse_indicator(
    mut image: DynamicImage,
    pixel_x: i32,
    pixel_y: i32,
    logical_x: i32,
    logical_y: i32,
    indicator_color: Rgba<u8>,
    label_color: Rgba<u8>,
) -> DynamicImage {
    let width = image.width();
    let height = image.height();
    
    // Clamp pixel position to image bounds
    let mx = pixel_x.max(0).min(width as i32);
    let my = pixel_y.max(0).min(height as i32);
    
    let mut img = image.to_rgba8();
    
    // Draw filled circle around cursor position
    let radius = 15;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx*dx + dy*dy <= radius*radius {
                let px = (mx + dx) as u32;
                let py = (my + dy) as u32;
                
                if px < width && py < height {
                    let pixel = img.get_pixel_mut(px, py);
                    let current = *pixel;
                    // Blend filled circle with existing pixel
                    let blend = 0.7; // 70% indicator, 30% original
                    let blended = Rgba([
                        ((current[0] as f32 * (1.0 - blend) + indicator_color[0] as f32 * blend) as u8),
                        ((current[1] as f32 * (1.0 - blend) + indicator_color[1] as f32 * blend) as u8),
                        ((current[2] as f32 * (1.0 - blend) + indicator_color[2] as f32 * blend) as u8),
                        current[3].max(indicator_color[3]),
                    ]);
                    *pixel = blended;
                }
            }
        }
    }
    
    // Draw crosshair on top
    let mut temp_img = DynamicImage::ImageRgba8(img).to_rgba8();
    draw_cross_mut(&mut temp_img, Rgba([255, 255, 255, 255]), mx, my); // White crosshair for contrast
    
    image = DynamicImage::ImageRgba8(temp_img);
    
    // Draw coordinate label with LOGICAL coordinates (what user uses for mouse.move)
    let label = format!("({}, {})", logical_x, logical_y);
    let char_w = 6u32;
    let char_h = 9u32;
    let label_w = (label.len() as u32 * char_w) + 8;
    let label_h = char_h + 4;
    
    // Position label above cursor
    let label_x = (mx as i32 - (label_w as i32 / 2)).max(2);
    let label_y = (my - radius as i32 - label_h as i32 - 5).max(2);
    
    // Background box
    let bg_color = Rgba([0, 0, 0, 220]);
    draw_filled_rect_mut(
        &mut image,
        Rect::at(label_x, label_y).of_size(label_w, label_h),
        bg_color,
    );
    
    // Draw text
    let mut char_x = label_x + 4;
    for ch in label.chars() {
        if ch.is_ascii_digit() || ch == '(' || ch == ')' || ch == ',' || ch == ' ' {
            if ch.is_ascii_digit() {
                draw_digit(&mut image, char_x, label_y + 2, ch, label_color);
            }
            char_x += char_w as i32;
        }
    }
    
    image
}
