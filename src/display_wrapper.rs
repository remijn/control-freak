use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::{IntoStorage, Rgb565},
    primitives::Rectangle,
    Pixel,
};
use esp_idf_svc::hal::{
    gpio::{Gpio38, Output, PinDriver},
    spi::{SpiDeviceDriver, SpiDriver},
};
use ssd1351::{interface::SpiInterface, mode::GraphicsMode};

// pub mod display_wrapper;
pub type DisplayType<'a> =
    GraphicsMode<SpiInterface<SpiDeviceDriver<'a, SpiDriver<'a>>, PinDriver<'a, Gpio38, Output>>>;

pub struct DisplayWrapper {
    pub display: DisplayType<'static>,
}

impl DisplayWrapper {
    /// Draw a `Pixel` that has a color defined as `Rgb565`.
    fn draw_pixel(
        &mut self,
        pixel: Pixel<Rgb565>,
    ) -> Result<(), <DisplayWrapper as DrawTarget>::Error> {
        let Pixel(coord, color) = pixel;

        // Place an (x, y) pixel at the right index in the framebuffer. If the pixel coordinates
        // are out of bounds (negative or greater than (127, 127)), this operation will be a
        // noop.
        if let Ok((x @ 0..=127, y @ 0..=127)) = coord.try_into() {
            self.display.set_pixel(x, y, color.into_storage())
        }

        Ok(())
    }
}

impl Dimensions for DisplayWrapper {
    fn bounding_box(&self) -> Rectangle {
        let (width, height) = self.display.get_dimensions();

        Rectangle::new(Point::new(0, 0), Size::new(width as u32, height as u32))
    }
}

impl DrawTarget for DisplayWrapper {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.draw_pixel(pixel)?
        }

        Ok(())
    }
}
