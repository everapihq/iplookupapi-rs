//! # iplookupapi-rs
//!
//! The [iplookupapi][iplookupapi] crate provides an easy to use wrapper over the
//! [iplookupapi api][iplookupapi_api].
//!
//! * Easy to use
//! * Async api calls with [reqwest][reqwest]
//! * Ready deserialized structs of the iplookupapi responses
//! * Manages authentication for you, just pass your api token once
//!
//! ## Requirements
//! * Your own [iplookupapi api key][iplookupapi_api]
//! * Async runtime configured e.g. [tokio][tokio]
//!
//!
//!
//! ## Examples
//!
//!
//! ## Optional Features
//!
//!
//! ## Troubleshooting
//! If you get a ResponseParsingError during usage of the crate this is very likely
//! due to an invalid input where the iplookupapi api will throw an error or
//! due to some unexpected values that were returned by the api. E.g. sometimes the api
//! will return `false` instead of a number for certain fields or other fields were missing.
//!
//! In this case please check if your input is valid and if so create a bug report on the
//! crate [repository][iplookupapi_rs_repo] and provide some information about your input.
//!
//! [iplookupapi]: ./api/struct.iplookupapi.html
//! [iplookupapirs_repo]: https://github.com/everapihq/iplookupapi-rs
//! [iplookupapi_api]: https://api.iplookupapi.com/
//! [reqwest]: https://crates.io/crates/reqwest
//! [tokio]: https://crates.io/crates/tokio

#![warn(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate serde;
extern crate reqwest;
extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate thiserror;

pub mod api;
mod error;
pub mod models;
mod utils;

pub use api::Iplookupapi;
pub use error::IplookupapiError as Error;
