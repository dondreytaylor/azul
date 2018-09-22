pub use app::RuntimeError;
pub use css::{CssParseError, DynamicCssParseError, HotReloadError};
pub use css_parser::{
    CssBackgroundParseError, CssBorderParseError, CssBorderRadiusParseError, CssColorParseError,
    CssDirectionParseError, CssFontFamilyParseError, CssGradientStopParseError, CssImageParseError,
    CssMetric, CssParsingError, CssShadowParseError, CssShapeParseError, InvalidValueErr,
    PercentageParseError, PixelParseError,
};
pub use font::FontError;
pub use image::ImageError;
pub use simplecss::Error as CssSyntaxError;

// TODO: re-export the sub-types of ClipboardError!
pub use clipboard2::ClipboardError;

pub use widgets::errors::*;
pub use window::WindowCreateError;

#[derive(Debug)]
pub enum Error<'a> {
    CssParse(CssParseError<'a>),
    Font(FontError),
    Image(ImageError),
    Clipboard(ClipboardError),
    WindowCreate(WindowCreateError),
    HotReload(HotReloadError),
}

impl<'a> From<CssParseError<'a>> for Error<'a> {
    fn from(e: CssParseError<'a>) -> Error {
        Error::CssParse(e)
    }
}

impl<'a> From<FontError> for Error<'a> {
    fn from(e: FontError) -> Error<'a> {
        Error::Font(e)
    }
}

impl<'a> From<ImageError> for Error<'a> {
    fn from(e: ImageError) -> Error<'a> {
        Error::Image(e)
    }
}

impl<'a> From<ClipboardError> for Error<'a> {
    fn from(e: ClipboardError) -> Error<'a> {
        Error::Clipboard(e)
    }
}

impl<'a> From<WindowCreateError> for Error<'a> {
    fn from(e: WindowCreateError) -> Error<'a> {
        Error::WindowCreate(e)
    }
}

impl<'a> From<HotReloadError> for Error<'a> {
    fn from(e: HotReloadError) -> Error<'a> {
        Error::HotReload(e)
    }
}

impl_display! {
    Error<'a>,
    {
        CssParse(e) => format!("[CSS parsing error] {}", e),
        Font(e) => format!("[Font error] {}", e),
        Image(e) => format!("[Image error] {}", e),
        Clipboard(e) => format!("[Clipboard error] {}", e),
        WindowCreate(e) => format!("[Window creation error] {}", e),
        HotReload(e) => format!("[Hot-reload error] {}", e),
    }
}
