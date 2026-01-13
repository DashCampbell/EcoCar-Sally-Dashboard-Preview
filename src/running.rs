use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::mono_font::iso_8859_13::FONT_10X20;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb666,
    prelude::{Point, RgbColor, Size},
    text::Text,
    Drawable,
};
use embedded_graphics_web_simulator::display::WebSimulatorDisplay;

pub fn running_gui(display: &mut WebSimulatorDisplay<Rgb666>) {
    let speed_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(25, 60))
        .digit_spacing(4) // 5px spacing between digits
        .segment_width(5) // 5px wide segments
        .segment_color(Rgb666::WHITE) // active segments are green
        .inactive_segment_color(Rgb666::BLACK)
        .build();

    // Render Running State
    display.clear(Rgb666::BLACK).unwrap();

    // Render Vehicle Speed
    Text::new(
        format!("{}", 40 % 100).as_str(),
        Point::new(260, 240),
        speed_style,
    )
    .draw(display)
    .unwrap();
    // Render speed unit
    Text::new(
        "km/h",
        Point::new(320, 240),
        MonoTextStyle::new(&FONT_10X20, Rgb666::WHITE),
    )
    .draw(display)
    .unwrap();

    display.flush().expect("could not flush buffer");
}
