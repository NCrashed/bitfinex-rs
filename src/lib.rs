#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces)]

#[macro_use] 
extern crate error_chain;
extern crate log;
extern crate hex;
extern crate ring;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate tungstenite;
extern crate url;

#[macro_use] 
extern crate serde_derive;

pub mod book;
pub mod client;
pub mod ticker;
pub mod trades;
pub mod orders;
pub mod account;
pub mod ledger;
pub mod auth;

pub mod candles;
pub mod api;
pub mod pairs;
pub mod currency;
pub mod precision;
pub mod websockets;
pub mod events;
pub mod errors;
