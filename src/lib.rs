//! An implementation of the [`embedded-graphics::DrawTarget`] trait using the [`flipdot`] crate
//! to provide an easy way to send text and graphics to Luminator flip-dot and LED signs over RS-485.
//!
//! Tested with a MAX3000 90 × 7 side sign. Should work with any flip-dot or LED sign that uses the 7-pin circular
//! connector, but no guarantees.
//!
//! Intended only for hobbyist and educational purposes. Not affiliated with Luminator in any way.
//!
//! # Examples
//!
//! ```no_run
//! use flipdot_graphics::{Address, FlipdotDisplay, SignBusType, SignType};
//!
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_5X7, MonoTextStyle},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, PrimitiveStyle, Triangle},
//!     text::{Baseline, Text},
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #
//! // Create a display for a sign connected over serial.
//! let mut display = FlipdotDisplay::try_new(
//!     SignBusType::Serial("/dev/ttyUSB0"),
//!     Address(3),
//!     SignType::Max3000Side90x7
//! )?;
//!
//! // Draw a circle and a triangle.
//! Circle::new(Point::new(2, 0), 6)
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .draw(&mut display)?;
//!
//! Triangle::new(Point::new(11, 1), Point::new(15, 5), Point::new(19, 1))
//!     .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
//!     .draw(&mut display)?;
//!
//! // Send the page to the sign.
//! display.flush()?;
//!
//! // Keep editing the same page, adding some text.
//! let style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);
//! Text::with_baseline("Hello, world!", Point::new(24, 0), style, Baseline::Top)
//!     .draw(&mut display)?;
//!
//! // Send the updated page to the sign.
//! display.flush()?;
//!
//! // Turn all pixels on.
//! display.clear(BinaryColor::On)?;
//! display.flush()?;
//!
//! // Turn all pixels off.
//! display.clear(BinaryColor::Off)?;
//! display.flush()?;
//! #
//! # Ok(()) }
//! ```
//!
//! [`embedded-graphics::DrawTarget`]: https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html
//! [`flipdot`]: https://docs.rs/flipdot
#![doc(html_root_url = "https://docs.rs/flipdot-graphics/0.1.0")]
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code
)]
#![warn(
    missing_docs,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

mod flipdot_display;

pub use self::flipdot_display::{FlipdotDisplay, SignBusType};

pub use flipdot::{Address, SignType};
