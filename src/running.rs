use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::mono_font::iso_8859_13::FONT_10X20;
use embedded_graphics::prelude::Dimensions;
use embedded_graphics::prelude::Transform;
use embedded_graphics::prelude::WebColors;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::{
    Circle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, StyledDrawable,
};
use embedded_graphics::text::TextStyleBuilder;
use embedded_graphics::{
    geometry::AnchorX,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb666,
    prelude::{Point, RgbColor, Size},
    text::{Alignment, Baseline, Text},
    Drawable,
};

use crate::DisplayDevice;

fn render_speed_widgets(display: &mut DisplayDevice, speed: i32) {
    let center_point = display.bounding_box().center();
    // Define Styles
    let speed_font_width = 25;
    let speed_font_height = 60;
    let speed_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(speed_font_width, speed_font_height))
        .digit_spacing(4) // 5px spacing between digits
        .segment_width(5) // 5px wide segments
        .segment_color(Rgb666::RED) // active segments are green
        .inactive_segment_color(Rgb666::BLACK)
        .build();
    let speed_unit_style = MonoTextStyle::new(&FONT_10X20, Rgb666::RED);
    let speed_circle_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb666::RED)
        .stroke_width(5)
        .stroke_alignment(StrokeAlignment::Outside)
        .build();
    let right_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Right)
        .baseline(Baseline::Bottom)
        .build();

    // Static Text
    Circle::with_center(center_point, 120)
        .draw_styled(&speed_circle_style, display)
        .unwrap();
    // Render Speed Unit
    Text::with_alignment(
        "km/h",
        center_point + Point::new(0, speed_font_height as i32 / 2 + 15),
        speed_unit_style,
        Alignment::Center,
    )
    .draw(display)
    .unwrap();

    // Dynamic Text
    Text::with_text_style(
        format!("{}", speed).as_str(),
        center_point + Point::new(speed_font_width as i32, speed_font_height as i32 / 2),
        speed_style,
        right_aligned,
    )
    .draw(display)
    .unwrap();
}

fn render_tach_widgets(display: &mut DisplayDevice, rpm: u32) {
    // Define Styles
    let center_point = display.bounding_box().center();
    let tach_line_width = 3;

    // The number of tach lines per 1000rpm
    let tach_lines = 5;
    // Maximum RPM Represented is 5000rpm
    let max_tach_lines = tach_lines * 5;

    let tach_empty_style = PrimitiveStyle::with_fill(Rgb666::CSS_SILVER);

    let tach_line_style = PrimitiveStyle::with_fill(Rgb666::RED);
    let tach_line = Rectangle::new(
        center_point.x_axis() - Point::new(max_tach_lines * tach_line_width * 2, -15),
        Size::new(tach_line_width as u32, 55),
    );

    let tach_divider_style = PrimitiveStyle::with_fill(Rgb666::CSS_DEEP_PINK);
    let tach_divider_line = tach_line.resized_width(tach_line_width as u32 + 2, AnchorX::Left);

    // Render Tachometer
    // Determines the distance between tachometer bars
    let tach_spacer = 4;
    // Maximum RPM Represented is 5000rpm
    let display_rpm = ((rpm as f32 / 5000f32) * max_tach_lines as f32) as i32;
    for i in 0..=display_rpm {
        let (bar, bar_style) = if (i % tach_lines) == 0 {
            (tach_divider_line, tach_divider_style)
        } else {
            (tach_line, tach_line_style)
        };
        bar.translate(Point::new(i * tach_line_width as i32 * tach_spacer, 0))
            .draw_styled(&bar_style, display)
            .unwrap();
    }
    for i in (display_rpm + 1)..=max_tach_lines {
        let tach_line = if (i % tach_lines) == 0 {
            tach_divider_line
        } else {
            tach_line
        };
        tach_line
            .translate(Point::new(i * tach_line_width as i32 * tach_spacer, 0))
            .draw_styled(&tach_empty_style, display)
            .unwrap();
    }
}

pub fn running_gui(display: &mut DisplayDevice) {
    ///////////////////////////////
    // Render Graphics
    ///////////////////////////////
    render_tach_widgets(display, 2650);
    render_speed_widgets(display, 40);
}
