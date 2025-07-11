//! # A2
//!
//! A2 is an asynchronous client to Apple push notification service. It
//! provides a type-safe way to generate correct requests, mapping responses into
//! corresponding types. The client supports both, certificate and token based
//! authentication.
//!
//! To create a connection it is required to have either a PKCS12 database file
//! including a valid certificate and private key with a password for unlocking
//! it, or a private key in PKCS8 PEM format with the corresponding team and key
//! ids. All of these should be available from Xcode or your Apple developer
//! account.
//!
//! ## Payload
//!
//! Building the notification payload should be done with the [DefaultNotificationBuilder](request/notification/struct.DefaultNotificationBuilder.html) for most use-cases.
//! There is also the [WebNotificationBuilder](request/notification/struct.WebNotificationBuilder.html) in the case you need to send notifications to safari
//!
//! The payload generated by the builder [can hold a custom data
//! section](request/payload/struct.Payload.html#method.add_custom_data),
//! defined by a selected root key. Any data using `#[derive(Serialize)]` from
//! [Serde](https://serde.rs/) works, allowing usage of type-safe structs or
//! dynamic hashmaps to generate the custom data.
//!
//! ## Client
//!
//! The [asynchronous client](client/struct.Client.html), works either with
//! [certificate](client/struct.Client.html#method.certificate) or
//! [token](client/struct.Client.html#method.token) authentication.
//!
//! ## Example sending a plain notification using token authentication:
//!
//! ```no_run
//! # use a2::{DefaultNotificationBuilder, NotificationBuilder, Client, ClientConfig, Endpoint};
//! # use std::fs::File;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let mut builder = DefaultNotificationBuilder::new()
//!     .set_body("Hi there")
//!     .set_badge(420)
//!     .set_category("cat1")
//!     .set_sound("ping.flac");
//!
//! let payload = builder.build("device-token-from-the-user", Default::default());
//! let mut file = File::open("/path/to/private_key.p8")?;
//!
//! let client = Client::token(
//!     &mut file,
//!     "KEY_ID",
//!     "TEAM_ID",
//!     ClientConfig::default()).unwrap();
//!
//! let response = client.send(payload).await?;
//! println!("Sent: {:?}", response);
//! # Ok(())
//! # }
//! ```
//!
//! ## Example sending a silent notification with custom data using certificate authentication:
//!
//! ```no_run
//! #[macro_use] extern crate serde;
//! # #[cfg(all(feature = "openssl", not(feature = "ring")))]
//! # {
//!
//! use a2::{
//!     Client, ClientConfig, Endpoint, DefaultNotificationBuilder, NotificationBuilder, NotificationOptions,
//!     Priority,
//! };
//! use std::fs::File;
//!
//! #[derive(Serialize, Debug)]
//! struct CorporateData {
//!     tracking_code: &'static str,
//!     is_paying_user: bool,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let tracking_data = CorporateData {
//!         tracking_code: "999-212-UF-NSA",
//!         is_paying_user: false,
//!     };
//!
//!     let mut payload = DefaultNotificationBuilder::new()
//!         .set_content_available()
//!         .build("device-token-from-the-user",
//!         NotificationOptions {
//!             apns_priority: Some(Priority::Normal),
//!             ..Default::default()
//!         },
//!     );
//!     payload.add_custom_data("apns_gmbh", &tracking_data)?;
//!
//!     let mut file = File::open("/path/to/cert_db.p12")?;
//!
//!     let client = Client::certificate(
//!         &mut file,
//!         "Correct Horse Battery Stable",
//!         ClientConfig::default())?;
//!
//!     let response = client.send(payload).await?;
//!     println!("Sent: {:?}", response);
//!
//!     Ok(())
//! }
//! # }
//! ```
#![warn(clippy::unwrap_used)]

#[cfg(not(any(feature = "openssl", feature = "ring")))]
compile_error!("either feature \"openssl\" or feature \"ring\" has to be enabled");

#[macro_use]
extern crate serde;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

pub mod client;
pub mod error;
mod pkcs12;
pub mod request;
pub mod response;
mod signer;

pub use crate::request::notification::{
    CollapseId, DefaultNotificationBuilder, NotificationBuilder, NotificationOptions, Priority, PushType,
    WebNotificationBuilder, WebPushAlert,
};

pub use crate::request::payload::InterruptionLevel;

pub use crate::response::{ErrorBody, ErrorReason, Response};

pub use crate::client::{Client, ClientConfig, Endpoint};

pub use crate::error::Error;
