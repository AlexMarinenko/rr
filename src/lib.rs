#[macro_use]
extern crate exonum;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate bodyparser;

extern crate iron;
extern crate router;
extern crate params;

#[macro_use]
extern crate log;
#[cfg(test)]
extern crate iron_test;
#[cfg(test)]
extern crate mime;
#[cfg(test)]
extern crate sandbox;

pub mod api;
pub mod blockchain;
mod service;

pub use service::{TimestampingService, TIMESTAMPING_SERVICE};

pub const TIMESTAMPING_TX_ID: u16 = 0;