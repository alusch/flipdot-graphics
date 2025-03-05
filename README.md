# flipdot-graphics

[![Build status](https://github.com/alusch/flipdot-graphics/workflows/build/badge.svg)](https://github.com/alusch/flipdot-graphics/actions)
[![Crates.io](https://img.shields.io/crates/v/flipdot-graphics.svg?logo=rust)](https://crates.io/crates/flipdot-graphics)
[![Docs.rs](https://img.shields.io/docsrs/flipdot-graphics.svg?logo=docs.rs)](https://docs.rs/flipdot-graphics)

An implementation of the [`embedded-graphics::DrawTarget`] trait using the [`flipdot`] crate to provide an easy way to send text and graphics to Luminator flip-dot and LED signs over RS-485.

Tested with a MAX3000 90 × 7 side sign. Should work with any flip-dot or LED sign that uses the 7-pin circular
connector, but no guarantees.

Intended only for hobbyist and educational purposes. Not affiliated with Luminator in any way.

## Usage

Here's a full example of drawing some graphics on a sign:

```rust
use flipdot_graphics::{Address, FlipdotDisplay, SignType};

use embedded_graphics::{
    mono_font::{ascii::FONT_5X7, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Triangle},
    text::Text,
};

// Connect to a sign with a given address and type over serial.
let mut display = FlipdotDisplay::try_new(
    "/dev/ttyUSB0",
    Address(3),
    SignType::Max3000Side90x7
)?;

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
```

## License

Distributed under the [MIT license].

[`embedded-graphics::DrawTarget`]: https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html
[`flipdot`]: https://github.com/alusch/flipdot
[MIT license]: /LICENSE
