use std::{cell::RefCell, iter, rc::Rc};

use embedded_graphics_core::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
};
use flipdot::{Address, Page, PageFlipStyle, PageId, SerialSignBus, Sign, SignBus, SignError, SignType};
use flipdot_testing::{VirtualSign, VirtualSignBus};

/// A [`DrawTarget`] implementation to easily draw graphics to a Luminator sign.
///
/// # Examples
///
/// ```no_run
/// use flipdot_graphics::{Address, FlipdotDisplay, SignBusType, SignType};
///
/// use embedded_graphics::{
///     mono_font::{MonoTextStyle, ascii::FONT_5X7},
///     pixelcolor::BinaryColor,
///     prelude::*,
///     primitives::{Circle, PrimitiveStyle, Triangle},
///     text::{Baseline, Text},
/// };
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// #
/// // Create a display for a sign connected over serial.
/// let mut display = FlipdotDisplay::try_new(
///     SignBusType::Serial("/dev/ttyUSB0"),
///     Address(3),
///     SignType::Max3000Side90x7
/// )?;
///
/// // Draw some shapes and text to the page.
/// Circle::new(Point::new(2, 0), 6)
///     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
///     .draw(&mut display)?;
///
/// Triangle::new(Point::new(11, 1), Point::new(15, 5), Point::new(19, 1))
///     .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
///     .draw(&mut display)?;
///
/// let style = MonoTextStyle::new(&FONT_5X7, BinaryColor::On);
/// Text::with_baseline("Hello, world!", Point::new(24, 0), style, Baseline::Top)
///     .draw(&mut display)?;
///
/// // Send the page to the sign to be displayed.
/// display.flush()?;
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct FlipdotDisplay {
    page: Page<'static>,
    sign: Sign,
}

/// The type of sign bus to create.
#[derive(Debug)]
pub enum SignBusType<'a> {
    /// Create a [`SerialSignBus`] for communicating with a real sign over the specified serial port.
    Serial(&'a str),

    /// Create a [`VirtualSignBus`] for testing.
    Virtual,
}

impl<'a, T: AsRef<str>> From<&'a T> for SignBusType<'a> {
    /// Pass "virtual" to use a virtual sign bus for testing, otherwise `value` will be interpreted as a serial port.
    fn from(value: &'a T) -> Self {
        let port = value.as_ref();
        if port.eq_ignore_ascii_case("virtual") {
            Self::Virtual
        } else {
            Self::Serial(port)
        }
    }
}

impl FlipdotDisplay {
    /// The easiest way to get started drawing to a sign in a standalone fashion.
    ///
    /// Creates a [`SignBus`] internally based on `bus_type` to simplify the common case.
    /// If you do need more control, you can provide your own bus using [`new_with_bus`](Self::new_with_bus).
    ///
    /// # Errors
    ///
    /// Returns the underlying [`serial::Error`] if the serial port cannot be configured.
    /// Virtual sign bus creation can never fail.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use flipdot_graphics::{Address, FlipdotDisplay, SignBusType, SignType};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// // Create a display for a sign connected over serial.
    /// let mut display = FlipdotDisplay::try_new(
    ///     SignBusType::Serial("COM3"),
    ///     Address(6),
    ///     SignType::Max3000Front98x16
    /// )?;
    /// #
    /// # Ok(()) }
    /// ```
    ///
    /// ```
    /// use flipdot_graphics::{Address, FlipdotDisplay, SignBusType, SignType};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// // Create a display for a virtual sign for testing
    /// // (set RUST_LOG=flipdot=info environment variable to see the results).
    /// let mut display = FlipdotDisplay::try_new(
    ///     SignBusType::Virtual,
    ///     Address(4),
    ///     SignType::HorizonDash40x12
    /// )?;
    /// #
    /// # Ok(()) }
    /// ```
    pub fn try_new(bus_type: SignBusType<'_>, address: Address, sign_type: SignType) -> Result<Self, serial::Error> {
        let bus: Rc<RefCell<dyn SignBus>> = match bus_type {
            SignBusType::Virtual => {
                let bus = VirtualSignBus::new(iter::once(VirtualSign::new(address, PageFlipStyle::Manual)));
                Rc::new(RefCell::new(bus))
            }
            SignBusType::Serial(port) => {
                let port = serial::open(port)?;
                let bus = SerialSignBus::try_new(port)?;
                Rc::new(RefCell::new(bus))
            }
        };

        Ok(Self::new_with_bus(bus, address, sign_type))
    }

    pub fn new_with_bus(bus: Rc<RefCell<dyn SignBus>>, address: Address, sign_type: SignType) -> Self {
        Sign::new(bus, address, sign_type).into()
    }

    /// Updates the display from the framebuffer.
    pub fn flush(&self) -> Result<(), SignError> {
        self.sign.configure_if_needed()?;

        if self.sign.send_pages(iter::once(&self.page))? == PageFlipStyle::Manual {
            self.sign.show_loaded_page()?;
        }

        Ok(())
    }
}

impl From<Sign> for FlipdotDisplay {
    fn from(sign: Sign) -> Self {
        Self {
            page: sign.create_page(PageId(0)),
            sign,
        }
    }
}

impl DrawTarget for FlipdotDisplay {
    type Color = BinaryColor;
    type Error = core::convert::Infallible; // Drawing itself can never fail since we just write to the Page.

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // `DrawTarget` contract requires ignoring out of bounds coordinates.
            if let Ok((x, y)) = coord.try_into() {
                let size = self.size();
                if x < size.width && y < size.height {
                    self.page.set_pixel(x, y, color.is_on());
                }
            }
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.page.set_all_pixels(color.is_on());
        Ok(())
    }
}

impl OriginDimensions for FlipdotDisplay {
    fn size(&self) -> Size {
        Size::new(self.sign.width(), self.sign.height())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::{
        prelude::*,
        primitives::{PrimitiveStyle, Triangle},
    };
    use std::error::Error;

    #[test]
    fn out_of_bounds_pixels() -> Result<(), Box<dyn Error>> {
        let bus = VirtualSignBus::new(iter::once(VirtualSign::new(Address(3), PageFlipStyle::Manual)));
        let bus = Rc::new(RefCell::new(bus));
        let mut display = FlipdotDisplay::new_with_bus(bus.clone(), Address(3), SignType::Max3000Side90x7);

        // Writing out of bounds shouldn't fail or panic
        display.draw_iter(iter::once(Pixel(Point::new(-1, 0), BinaryColor::On)))?;
        display.draw_iter(iter::once(Pixel(Point::new(0, -1), BinaryColor::On)))?;
        display.draw_iter(iter::once(Pixel(Point::new(90, 0), BinaryColor::On)))?;
        display.draw_iter(iter::once(Pixel(Point::new(0, 7), BinaryColor::On)))?;
        display.flush()?;

        // And should result in an empty page
        let bus = bus.borrow();
        let page = &bus.sign(0).pages()[0];
        assert_eq!(*page, Page::new(page.id(), page.width(), page.height()));

        Ok(())
    }

    #[test]
    fn draw() -> Result<(), Box<dyn Error>> {
        let bus = VirtualSignBus::new(iter::once(VirtualSign::new(Address(3), PageFlipStyle::Manual)));
        let bus = Rc::new(RefCell::new(bus));
        let mut display = FlipdotDisplay::new_with_bus(bus.clone(), Address(3), SignType::Max3000Side90x7);

        Triangle::new(Point::new(0, 0), Point::new(45, 6), Point::new(89, 0))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)?;

        display.flush()?;

        let actual = format!("{}", bus.borrow().sign(0).pages()[0]);
        let expected = "\
            +------------------------------------------------------------------------------------------+\n\
            |@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@|\n\
            |    @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@    |\n\
            |            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@            |\n\
            |                   @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                   |\n\
            |                           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@                          |\n\
            |                                  @@@@@@@@@@@@@@@@@@@@@@                                  |\n\
            |                                          @@@@@@@                                         |\n\
            +------------------------------------------------------------------------------------------+";

        assert_eq!(actual, expected);

        Ok(())
    }
}
