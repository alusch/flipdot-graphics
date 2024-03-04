use std::error::Error;

use flipdot::{Address, SignType};
use flipdot_graphics::{FlipdotDisplay, VIRTUAL_SIGN};

use embedded_graphics::{
    mono_font::{ascii::FONT_5X7, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Triangle},
    text::Text,
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut display = FlipdotDisplay::try_new(VIRTUAL_SIGN, Address(3), SignType::Max3000Side90x7)?;

    Circle::new(Point::new(2, 0), 7)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)?;

    Triangle::new(Point::new(11, 1), Point::new(15, 5), Point::new(19, 1))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)?;

    display.flush()?;

    let style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);
    Text::new("Hey there!", Point::new(22, 5), style).draw(&mut display)?;

    display.flush()?;

    Ok(())
}
