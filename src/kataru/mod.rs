#[macro_use]
pub mod error;
pub mod comparator;
pub mod conditional;
pub mod operator;
pub mod runner;
pub mod state;
pub mod structs;
pub mod validate;
pub mod value;

pub use error::ValidationError;
pub use runner::Runner;
pub use structs::{Config, Passage, PassageLine, Story};
pub use validate::validate;
