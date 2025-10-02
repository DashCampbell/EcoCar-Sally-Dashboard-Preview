# EcoCar-Sally-Dashboard-Preview

Preview accessed here:
https://dashcampbell.github.io/EcoCar-Sally-Dashboard-Preview/

Preview of what the dashboard will look like for Sally. Currently a work in progress. The dashboard
will have at least 4 states. Running, Charging, Standby and Startup mode.

This uses [embedded-graphics-web-simulator
](https://crates.io/crates/embedded-graphics-web-simulator) to compile code to web assembly files, which is then deployed to GitHub pages using GitHub actions. The simulator uses the same graphics library as the real project, [embedded-graphics](https://crates.io/crates/embedded-graphics). The real project will use an [ILI9488](https://crates.io/crates/ili9488-rs) display driver.

Notes:
* Numeric text that changes on each frame is rendered using the [seven-segment-display](https://crates.io/crates/eg-seven-segment) font to optimize rendering speed. The reason for this is because the ILI9488 is optimized for rendering rectangles and the seven-segment font is rendered using multiple horizontal/veritcal lines (rectangles), as seen [here](https://github.com/embedded-graphics/eg-seven-segment/blob/master/src/segment.rs#L39)
* Embedded rust does not use any heap memory by default. Therefore the demo should not rely on data structures such as `Vec` of `String`.