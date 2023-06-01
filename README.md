<p align="center">
<img src="https://app.iplookupapi.com/img/logo/iplookupapi.png" width="300"/>
</p>

# iplookupapi-rs: Rust geolocation service via iplookupapi.com

This package is a Rust wrapper for [iplookupapi.com](https://iplookupapi.com) that aims to make the usage of the API as easy as possible in your project.


## Installation

This crate is under development. Especially the response parsing needs some more testing. However, if you still want to use it, you can install it by adding this to your `Cargo.toml`:

```toml
[dependencies]
iplookupapi = "0.1.0"
```

## Requirements

1. API Key for [iplookupapi.com](https://iplookupapi.com/)
2. Async runtime like [tokio](https://crates.io/crates/tokio)

## Quickstart

```rust
use iplookupapi::iplookupapi;
use iplookupapi::models;

async fn request_latest() -> Result<models::DetailsResponse, iplookupapi::Error> {
    let iplookupapi_api = iplookupapi::new("<your-api-key>")?;
    let details = iplookupapi_api.info("1.1.1.1").await?;
     Ok(details)
}
```

Find out more about our endpoints, parameters and response data structure in the [docs]

## License

The MIT License (MIT). Please see [License File](LICENSE.md) for more information.

[docs]: https://iplookupapi.com/docs
[iplookupapi.com]: https://iplookupapi.com