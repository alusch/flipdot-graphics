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
