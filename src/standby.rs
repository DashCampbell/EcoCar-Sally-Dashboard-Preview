use std::sync::{Arc, Mutex};

/* Canbus values to render
pub static RELAY_STATE: Mutex<ThreadModeRawMutex, RelayState> = Mutex::new(RelayState::RELAY_STRTP);

pub static FET_DATA: Mutex<ThreadModeRawMutex, FDCAN_FetPack_t> = Mutex::new(FDCAN_FetPack_t {
    fet_config: 0,
    input_volt: 0,
    cap_volt: 0,
    cap_curr: 0,
    res_curr: 0,
    out_curr: 0,
});

pub static FCC_PACK1_DATA: Mutex<ThreadModeRawMutex, FDCAN_FccPack1_t> =
    Mutex::new(FDCAN_FccPack1_t {
        fc_press: 0,
        fc_temp: 0,
    });
pub static FCC_PACK2_DATA: Mutex<ThreadModeRawMutex, FDCAN_FccPack2_t> =
    Mutex::new(FDCAN_FccPack2_t {
        fan_rpm1: 0,
        fan_rpm2: 0,
    });
pub static FCC_PACK3_DATA: Mutex<ThreadModeRawMutex, FDCAN_FccPack3_t> =
    Mutex::new(FDCAN_FccPack3_t {
        bme_temp: 0,
        bme_humid: 0,
    });

pub static H2_PACK1_DATA: Mutex<ThreadModeRawMutex, ECOCAN_H2Pack1_t> =
    Mutex::new(ECOCAN_H2Pack1_t {
        h2_sense_1: 0,
        h2_sense_2: 0,
        h2_sense_3: 0,
        h2_sense_4: 0,
    });
pub static H2_PACK2_DATA: Mutex<ThreadModeRawMutex, ECOCAN_H2Pack2_t> =
    Mutex::new(ECOCAN_H2Pack2_t {
        bme_temp: 0,
        bme_humid: 0,
        imon_7v: 0,
        imon_12v: 0,
    });

pub static BOOST_PACK1_DATA: Mutex<ThreadModeRawMutex, FDCAN_BOOSTPack1_t> =
    Mutex::new(FDCAN_BOOSTPack1_t {
        in_curr: 0,
        in_volt: 0,
    });
pub static BOOST_PACK2_DATA: Mutex<ThreadModeRawMutex, FDCAN_BOOSTPack2_t> =
    Mutex::new(FDCAN_BOOSTPack2_t {
        out_curr: 0,
        out_volt: 0,
    });
pub static BOOST_PACK3_DATA: Mutex<ThreadModeRawMutex, FDCAN_BOOSTPack3_t> =
    Mutex::new(FDCAN_BOOSTPack3_t {
        efficiency: 0,
        joules: 0,
    });

/// Fuel Cell Reading
pub static REL_FC_PACK: Mutex<ThreadModeRawMutex, FDCAN_RelPackFc_t> =
    Mutex::new(FDCAN_RelPackFc_t {
        fc_volt: 0,
        fc_curr: 0,
    });
pub static REL_CAP_PACK: Mutex<ThreadModeRawMutex, FDCAN_RelPackCap_t> =
    Mutex::new(FDCAN_RelPackCap_t {
        cap_volt: 0,
        cap_curr: 0,
    });
pub static RELAY_MOTOR_PACK: Mutex<ThreadModeRawMutex, FDCAN_RelPackMtr_t> =
    Mutex::new(FDCAN_RelPackMtr_t {
        mtr_volt: 0,
        mtr_curr: 0,
    }); */
use crate::{DisplayDevice, CENTER_POINT};
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::mono_font::iso_8859_1::{FONT_8X13, FONT_9X15};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::*,
    text::{Alignment, Text},
    Drawable,
};

const MAX_ROWS_PER_COLUMN: i32 = 16;
static CURRENT_ROW: Mutex<i32> = Mutex::new(0);

fn render_can_value(field: &str, value: u32, display: &mut DisplayDevice) {
    let value = format!("{}", value);
    let field = format!("{}: ", field);

    let font = FONT_9X15;
    let font_width = font.character_size.width;
    let font_height = font.character_size.height;

    let number_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(font_width, font_height))
        .digit_spacing(2)
        .segment_width(1)
        .segment_color(Rgb666::WHITE)
        .inactive_segment_color(Rgb666::BLACK)
        .build();
    let text_style = MonoTextStyle::new(&font, Rgb666::WHITE);
    let mut row = CURRENT_ROW.lock().unwrap();
    let col = if *row >= MAX_ROWS_PER_COLUMN { 1 } else { 0 };

    let text_pos = Point::new(
        font_width as i32 * 14 + col * CENTER_POINT.x,
        20 + (font_height as i32 + 4) * (*row - col * MAX_ROWS_PER_COLUMN),
    );

    // Rendering
    let text = Text::with_alignment(&field, text_pos, text_style, Alignment::Right);
    text.draw(display).unwrap();
    let number = Text::with_alignment(&value, text_pos, number_style, Alignment::Left);
    number.draw(display).unwrap();

    // Increment Row number by one
    *row += 1;
}

pub fn standby_gui(display: &mut DisplayDevice, frame_index: u32) {
    let mock_value = 1234567000 + frame_index;

    // RELAY_STATE
    render_can_value("relay_state", mock_value, display);

    // FET_DATA
    render_can_value("fet_config", mock_value, display);
    render_can_value("input_volt", mock_value, display);
    render_can_value("cap_volt", mock_value, display);
    render_can_value("cap_curr", mock_value, display);
    render_can_value("res_curr", mock_value, display);
    render_can_value("out_curr", mock_value, display);

    // FCC_PACK1_DATA
    render_can_value("fc_press", mock_value, display);
    render_can_value("fc_temp", mock_value, display);

    // FCC_PACK2_DATA
    render_can_value("fan_rpm1", mock_value, display);
    render_can_value("fan_rpm2", mock_value, display);

    // // FCC_PACK3_DATA
    // render_can_value("bme_temp", mock_value, display);
    // render_can_value("bme_humid", mock_value, display);

    // H2_PACK1_DATA
    render_can_value("h2_sense_1", mock_value, display);
    render_can_value("h2_sense_2", mock_value, display);
    render_can_value("h2_sense_3", mock_value, display);
    render_can_value("h2_sense_4", mock_value, display);

    // H2_PACK2_DATA
    render_can_value("bme_temp", mock_value, display);
    render_can_value("bme_humid", mock_value, display);
    render_can_value("imon_7v", mock_value, display);
    render_can_value("imon_12v", mock_value, display);

    // BOOST_PACK1_DATA
    render_can_value("in_curr", mock_value, display);
    render_can_value("in_volt", mock_value, display);

    // BOOST_PACK2_DATA
    render_can_value("out_curr", mock_value, display);
    render_can_value("out_volt", mock_value, display);

    // BOOST_PACK3_DATA
    render_can_value("efficiency", mock_value, display);
    render_can_value("joules", mock_value, display);

    // REL_FC_PACK
    render_can_value("fc_volt", mock_value, display);
    render_can_value("fc_curr", mock_value, display);

    // REL_CAP_PACK
    render_can_value("cap_volt", mock_value, display);
    render_can_value("cap_curr", mock_value, display);

    // REL_MOTOR_PACK
    render_can_value("mtr_volt", mock_value, display);
    render_can_value("mtr_curr", mock_value, display);

    // Reset Row number after each frame
    let mut row = CURRENT_ROW.lock().unwrap();
    *row = 0;
}
