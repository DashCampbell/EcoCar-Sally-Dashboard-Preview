use crate::{DisplayDevice, CENTER_POINT};
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::mono_font::iso_8859_13::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::WebColors;
use embedded_graphics::primitives::StyledDrawable;
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
    primitives::{Arc, PrimitiveStyle},
    text::{Alignment, Text},
    Drawable,
};

fn render_battery_voltage_gui(display: &mut DisplayDevice, batt_voltage: f32) {
    // Define Styles
    let batt_font_width = 20;
    let batt_font_height = 35;
    let batt_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(batt_font_width, batt_font_height))
        .digit_spacing(3) // 5px spacing between digits
        .segment_width(4) // 5px wide segments
        .segment_color(Rgb666::WHITE) // active segments are green
        .inactive_segment_color(Rgb666::BLACK)
        .build();
    let batt_unit_style = MonoTextStyle::new(&FONT_10X20, Rgb666::WHITE);

    // Static Renders
    // Render Speed Unit
    Text::with_alignment(
        "V",
        CENTER_POINT
            + Point::new(
                batt_font_width as i32 * 2 + 10 + 5,
                batt_font_height as i32 / 2,
            ),
        batt_unit_style,
        Alignment::Right,
    )
    .draw(display)
    .unwrap();

    // Dynamic Text
    Text::with_alignment(
        format!("{:.1}", batt_voltage).as_str(),
        CENTER_POINT + Point::new(batt_font_width as i32 * 2, batt_font_height as i32 / 2),
        batt_style,
        Alignment::Right,
    )
    .draw(display)
    .unwrap();
}

fn render_battery_meter_gui(display: &mut DisplayDevice, frame_index: u32) {
    let angle_start = 130f32;
    let arc_diameter = 160;
    let border_width = 2;

    let border_style = PrimitiveStyle::with_stroke(Rgb666::CSS_DARK_GRAY, 12 + border_width * 2);
    let empty_style = PrimitiveStyle::with_stroke(Rgb666::BLACK, 12);
    let fill_style = PrimitiveStyle::with_stroke(Rgb666::GREEN, 12);

    Arc::with_center(
        CENTER_POINT,
        arc_diameter,
        (angle_start - border_width as f32).deg(),
        (360.0 - (angle_start - 90.0) * 2.0 + border_width as f32 * 2.0).deg(),
    )
    .draw_styled(&border_style, display)
    .unwrap();

    Arc::with_center(
        CENTER_POINT,
        arc_diameter,
        angle_start.deg(),
        (360.0 - (angle_start - 90.0) * 2.0).deg(),
    )
    .draw_styled(&empty_style, display)
    .unwrap();

    Arc::with_center(
        CENTER_POINT,
        arc_diameter,
        angle_start.deg(),
        ((360.0 - (angle_start - 90.0) * 2.0) * (frame_index as f32 / 100f32)).deg(),
    )
    .draw_styled(&fill_style, display)
    .unwrap();
}

pub fn charging_gui(display: &mut DisplayDevice, frame_index: u32) {
    let batt_voltage = 48f32 * (frame_index as f32 / 100f32);
    render_battery_voltage_gui(display, batt_voltage);
    render_battery_meter_gui(display, frame_index);
}
