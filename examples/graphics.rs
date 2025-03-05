use std::error::Error;

use flipdot_graphics::{Address, FlipdotDisplay, SignType, VIRTUAL_SIGN};

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_5X7},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Triangle},
    text::Text,
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Connect to a virtual sign for demonstration purposes (set RUST_LOG=flipdot=info environment variable to see the results).
    let mut display = FlipdotDisplay::try_new(VIRTUAL_SIGN, Address(3), SignType::Max3000Side90x7)?;

    // Draw a circle and a triangle.
    Circle::new(Point::new(2, 0), 7)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)?;

    Triangle::new(Point::new(11, 1), Point::new(15, 5), Point::new(19, 1))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)?;

    // Send the page to the sign.
    display.flush()?;

    // Keep editing the same page, adding some text.
    let style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);
    Text::new("Hey there!", Point::new(22, 5), style).draw(&mut display)?;

    // Send the updated page to the sign.
    display.flush()?;

    // Set all pixels on.
    display.clear(BinaryColor::On)?;
    display.flush()?;

    Ok(())
}
