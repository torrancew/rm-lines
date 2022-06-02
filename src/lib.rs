//! Binary parser for v5 of the reMarkable .lines binary format
//!
//! While similar to (and inspired by) [`lines-are-rusty`][rusty-lines],
//! this crate focuses *solely* on parsing the `.rm` files generated
//! by a reMarkable tablet (eg an individual notebook page). Other functionality
//! should be layered in higher-level crates.
//!
//! [rusty-lines]: https://github.com/ax3l/lines-are-rusty

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

/// Attempts to parse the implementing type from a byte sequence
///
/// This trait is currently built on the [nom][nom] parser combinator library,
/// and leaks details of this implementation. As such, it is [sealed][sealed-traits].
///
/// [nom]: https://docs.rs/nom/latest/nom/
/// [sealed-traits]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
pub trait Parse<'i>: private::Sealed {
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self>;
}
