pub mod error;
pub mod handle;
pub mod server;
pub mod util;

pub use self::server::{capabilities, config, session};

fn init() {
    println!("Hello, world!");
}
