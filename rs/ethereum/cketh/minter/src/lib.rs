pub mod address;
mod cbor;
pub mod checked_amount;
pub mod endpoints;
pub mod eth_logs;
pub mod eth_rpc;
pub mod eth_rpc_client;
pub mod eth_rpc_error;
pub mod eventlog;
pub mod guard;
pub mod lifecycle;
pub mod logs;
pub mod management;
pub mod map;
pub mod numeric;
mod serde_data;
pub mod state;
pub mod transactions;
pub mod tx;

#[cfg(test)]
mod tests;

use serde_bytes::ByteBuf;

pub const MAIN_DERIVATION_PATH: Vec<ByteBuf> = vec![];
