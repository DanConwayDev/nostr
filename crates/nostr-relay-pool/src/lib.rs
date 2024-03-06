// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Nostr Relay Pool

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![allow(unknown_lints)]
#![allow(clippy::arc_with_non_send_sync)]

pub mod pool;
pub mod relay;
mod util;

pub use self::pool::options::RelayPoolOptions;
pub use self::pool::{RelayPool, RelayPoolNotification};
pub use self::relay::flags::{AtomicRelayServiceFlags, RelayServiceFlags};
pub use self::relay::limits::Limits;
pub use self::relay::options::{
    FilterOptions, NegentropyDirection, NegentropyOptions, RelayOptions, RelaySendOptions,
    RequestAutoCloseOptions, RequestOptions,
};
pub use self::relay::stats::RelayConnectionStats;
pub use self::relay::{
    ActiveSubscription, InternalSubscriptionId, Relay, RelayNotification, RelayStatus,
};
