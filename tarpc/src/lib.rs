// Copyright 2016 Google Inc. All Rights Reserved.
//
// Licensed under the MIT License, <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

//! An RPC library for Rust.
//!
//! Example usage:
//!
//! ```
//! # #![feature(custom_derive, plugin)]
//! # #![plugin(serde_macros)]
//! # #[macro_use] extern crate tarpc;
//! # extern crate serde;
//! mod my_server {
//!     service! {
//!         rpc hello(name: String) -> String;
//!         rpc add(x: i32, y: i32) -> i32;
//!     }
//! }
//!
//! use self::my_server::*;
//! use std::time::Duration;
//!
//! struct Server;
//! impl my_server::Service for Server {
//!     fn hello(&self, s: String) -> String {
//!         format!("Hello, {}!", s)
//!     }
//!     fn add(&self, x: i32, y: i32) -> i32 {
//!         x + y
//!     }
//! }
//!
//! fn main() {
//!     let addr = "127.0.0.1:9000";
//!     let shutdown = my_server::serve(addr,
//!                                     Server,
//!                                     Some(Duration::from_secs(30)))
//!                               .unwrap();
//!     let client = Client::new(addr, None).unwrap();
//!     assert_eq!(3, client.add(1, 2).unwrap());
//!     assert_eq!("Hello, Mom!".to_string(),
//!                client.hello("Mom".to_string()).unwrap());
//!     drop(client);
//!     shutdown.shutdown();
//! }
//! ```

#![deny(missing_docs)]
#![feature(custom_derive, plugin, test, type_ascription)]
#![plugin(serde_macros)]

extern crate serde;
extern crate bincode;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate scoped_pool;
extern crate test;

macro_rules! pos {
    () => (concat!(file!(), ":", line!()))
}

/// Provides the tarpc client and server, which implements the tarpc protocol.
/// The protocol is defined by the implementation.
pub mod protocol;

/// Provides the macro used for constructing rpc services and client stubs.
pub mod macros;

pub use protocol::{Error, Result, ServeHandle};
