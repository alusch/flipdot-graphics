use std::{cell::RefCell, ffi::OsStr, iter, rc::Rc};

use embedded_graphics_core::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::BinaryColor,
};
use flipdot::{Address, Page, PageFlipStyle, PageId, SerialSignBus, Sign, SignBus, SignError, SignType};
use flipdot_testing::{VirtualSign, VirtualSignBus};

pub const VIRTUAL_SIGN: &str = "virtual";

#[derive(Debug)]
pub struct FlipdotDisplay {
    page: Page<'static>,
    sign: Sign,
}

impl FlipdotDisplay {
    pub fn try_new<T: AsRef<OsStr> + ?Sized>(port: &T, address: Address, sign_type: SignType) -> Result<Self, serial::Error> {
        let bus: Rc<RefCell<dyn SignBus>> = if port.as_ref().eq_ignore_ascii_case(VIRTUAL_SIGN) {
            let bus = VirtualSignBus::new(iter::once(VirtualSign::new(address, PageFlipStyle::Manual)));
            Rc::new(RefCell::new(bus))
        } else {
            let port = serial::open(port)?;
            let bus = SerialSignBus::try_new(port)?;
            Rc::new(RefCell::new(bus))
        };

        Ok(Self::new_with_bus(bus, address, sign_type))
    }

    pub fn new_with_bus(bus: Rc<RefCell<dyn SignBus>>, address: Address, sign_type: SignType) -> Self {
        Sign::new(bus, address, sign_type).into()
    }

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
