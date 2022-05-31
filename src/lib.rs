mod color;
pub use color::Color;

mod header;
pub(crate) use header::Header;

mod layer;
pub use layer::Layer;

mod line;
pub use line::Line;

mod page;
pub use page::Page;

mod point;
pub use point::Point;

mod tool;
pub use tool::Tool;

mod version;
pub use version::Version;

mod private {
    pub trait Sealed: Sized {}
    impl Sealed for crate::Color {}
    impl Sealed for crate::Layer {}
    impl Sealed for crate::Header {}
    impl Sealed for crate::Line {}
    impl Sealed for crate::Page {}
    impl Sealed for crate::Point {}
    impl Sealed for crate::Tool {}
    impl Sealed for crate::Version {}
}

pub trait Parse<'i>: private::Sealed {
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self>;
}
