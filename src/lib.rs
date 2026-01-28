use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::primitives::{Rectangle, StyledDrawable};
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod assets;
pub mod charging;
pub mod running;
pub mod standby;
pub mod startup;

// use crate::assets::*;
use crate::charging::{charging_gui, init_render_charging_gui};
use crate::running::{init_render_running_gui, running_gui};
use crate::standby::standby_gui;
use crate::startup::startup_gui;
use embedded_graphics::{
    pixelcolor::Rgb666,
    prelude::{Point, RgbColor, Size},
    primitives::PrimitiveStyle,
};

type DisplayDevice = WebSimulatorDisplay<Rgb666>;
const DISPLAY_WIDTH: u32 = 480;
const DISPLAY_HEIGHT: u32 = 320;
const CENTER_POINT: Point = Point::new(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32 / 2);

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn _render_rgb565_image<D>(
    display: &mut D,
    x0: u16,
    y0: u16,
    original_width: u16,
    screen_width: u16,
    data: &[u16],
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb666>,
{
    let pixel_size = screen_width / original_width;
    let mut x = 0;
    let mut y = 0;
    for line in data.chunks_exact(original_width as usize) {
        for pixel in line {
            let point = Rectangle::new(
                Point::new(x + x0 as i32, y + y0 as i32) * pixel_size as i32,
                Size::new(pixel_size as u32, pixel_size as u32),
            );
            let color = PrimitiveStyle::with_fill(Rgb666::new(
                ((pixel & 0xF800) >> 10) as u8,
                ((pixel & 0x07E0) >> 5) as u8,
                (pixel & 0x001F << 1) as u8,
            ));
            point.draw_styled(&color, display)?;

            x += 1;
        }
        x = 0;
        y += 1;
    }
    Ok(())
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let document = web_sys::window()
        .expect("could not get window")
        .document()
        .expect("could not get document");
    let body = document.body().expect("could not get document body");

    // for simplicity reasons, this example uses `cargo-run-wasm`, which doesn't allow
    // custom html - so it's augmented here inline. In a real project, you'd likely use `trunk` instead.
    body.set_inner_html(
        r#"
    <header>
    EcoCar Sally Dashboard Preview (WIP)
  </header>

  <div id="running-mode" class="display">
  <h4>Running Mode</h4>
  </div>
  
  <div id="standby-mode" class="display">
  <h4>Standby Mode</h4>
  </div>

  <div id="charging-mode" class="display">
  <h4>Charging Mode</h4>
  </div>
  
  <div id="startup-mode" class="display">
  <h4>Startup Mode</h4>
  </div>
  
  <footer>
  <p>Source Code: <a href="https://github.com/DashCampbell/EcoCar-Sally-Dashboard-Preview">https://github.com/DashCampbell/EcoCar-Sally-Dashboard-Preview</a></p>
  </footer>
    "#,
    );

    let output_settings = OutputSettingsBuilder::new()
        .scale(1)
        .pixel_spacing(0)
        .build();

    let mut startup_display: DisplayDevice = WebSimulatorDisplay::new(
        (DISPLAY_WIDTH, DISPLAY_HEIGHT),
        &output_settings,
        document.get_element_by_id("startup-mode").as_ref(),
    );
    let mut standby_display: DisplayDevice = WebSimulatorDisplay::new(
        (DISPLAY_WIDTH, DISPLAY_HEIGHT),
        &output_settings,
        document.get_element_by_id("standby-mode").as_ref(),
    );
    let mut charging_display: DisplayDevice = WebSimulatorDisplay::new(
        (DISPLAY_WIDTH, DISPLAY_HEIGHT),
        &output_settings,
        document.get_element_by_id("charging-mode").as_ref(),
    );
    let mut running_display: DisplayDevice = WebSimulatorDisplay::new(
        (DISPLAY_WIDTH, DISPLAY_HEIGHT),
        &output_settings,
        document.get_element_by_id("running-mode").as_ref(),
    );

    // Array of all bitmaps for convenience. (Total bytes used to store images in PROGMEM = 4416)
    // let _nyan_frames = [
    //     &epd_bitmap_nyan1,
    //     &epd_bitmap_nyan2,
    //     &epd_bitmap_nyan3,
    //     &epd_bitmap_nyan4,
    //     &epd_bitmap_nyan5,
    //     &epd_bitmap_nyan6,
    // ];

    // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // number of times. After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    //
    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 10;
    let mut increase_index = true;
    const NUM_ITER: i32 = 99;

    // Initial Frame
    let _ = startup_display.clear(Rgb666::BLACK);
    let _ = standby_display.clear(Rgb666::BLACK);
    let _ = charging_display.clear(Rgb666::BLACK);
    let _ = running_display.clear(Rgb666::BLACK);

    init_render_running_gui(&mut running_display);
    running_display.flush().expect("could not flush buffer");

    startup_gui(&mut startup_display);
    startup_display.flush().expect("could not flush buffer");

    init_render_charging_gui(&mut charging_display);
    charging_display.flush().expect("could not flush buffer");

    standby_gui(&mut standby_display, true, i);
    standby_display.flush().expect("could not flush buffer");

    // Rendering Loop
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        standby_gui(&mut standby_display, false, i);
        charging_gui(&mut charging_display, i);
        running_gui(&mut running_display, i);

        standby_display.flush().expect("could not flush buffer");
        charging_display.flush().expect("could not flush buffer");
        running_display.flush().expect("could not flush buffer");

        if increase_index {
            i += 1;
        } else {
            i -= 1;
        }
        if i > NUM_ITER as u32 {
            i = NUM_ITER as u32;
            increase_index = false;
        } else if i <= 10 {
            increase_index = true;
        }
        // if i > NUM_ITER {
        //     // Drop our handle to this closure so that it will get cleaned
        //     // up once we return.
        //     let _ = f.borrow_mut().take();
        //     return;
        // }
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
