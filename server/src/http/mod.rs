// The mod.rs file is for the interface of other mods.

pub use method::Method; // Meaning for Method from method.rs
pub use request::ParseError;
pub use request::Request;

// It can expose the submodules of method
pub mod method;
pub mod request;
