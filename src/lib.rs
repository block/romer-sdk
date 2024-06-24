// This file is Copyright its original authors, visible in version contror
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

#![crate_name = "romer"]

//! # Romer
//! A ready-to-go library for integrating lightning payments into your application.
//!
//! Romer is a non-custodial Lightning node in library form. Its central goal is to provide a
//! small, simple, and straightforward interface that enables users to easily send and receive
//! payments over the Lightning network.
//!
//! ## Getting Started
//!
//! The primary abstraction of the library is [`Romer`], which can be constructed by providing
//! your c= api key to [`new`]. `Romer` can then be
//! controlled via commands such as [`send`], [`receive`], [`list_payments`],
//! [`balance`], etc.:
//!
//! ```no_run
//! use romer::Romer;
//!
//! fn main() {
//! 	let romer = Romer::new("my-api-key").unwrap();
//!
//! 	let invoice = romer.receive(100_000, "alpaca socks").unwrap();
//!
//! 	romer.send("INVOICE_STR").unwrap();
//!
//! 	let payments = romer.list_payments();
//! 	let balance = romer.balance();
//! }
//! ```
//!
//! [`new`]: Romer::new
//! [`send`]: Romer::send
//! [`receive`]: Romer::receive
//! [`list_payments`]: Romer::list_payments
//! [`balance`]: Romer::balance
#![cfg_attr(not(feature = "uniffi"), deny(missing_docs))]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![allow(bare_trait_objects)]
#![allow(ellipsis_inclusive_range_patterns)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod error;
mod services;
mod token;
#[cfg(feature = "uniffi")]
mod uniffi_types;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{mpsc, Arc, Mutex};

use error::Error;
pub use error::Error as RomerError;
use ldk_node::bitcoin::BlockHash;
use ldk_node::lightning::ln::channelmanager::PaymentId;
use ldk_node::{
	bitcoin::{hashes::Hash, Address, Network, Txid},
	lightning::{events::PaymentFailureReason, ln::PaymentHash},
	lightning_invoice::Bolt11Invoice,
	payment::PaymentDetails,
	Builder, Node,
};
use services::ServiceConfig;
#[cfg(feature = "uniffi")]
use uniffi_types::*;

#[cfg(feature = "uniffi")]
uniffi::include_scaffolding!("romer");

const DEFAULT_INVOICE_EXPIRY_SECS: u32 = 3600;

/// The current balances across onchain and lightning wallets
pub struct Balances {
	/// How many sats are currently spendable onchain.
	pub spendable_onchain_balance_sats: u64,
	/// The total onchain balance in sats.
	pub total_onchain_balance_sats: u64,
	/// How many sats are held in reserve to pay for fee bumping.
	pub total_anchor_channels_reserve_sats: u64,
	/// How many sats we hold in lightning channels.
	pub total_lightning_balance_sats: u64,
	/// How many sats we can send over lightning.
	pub outbound_capacity_lightning_sats: u64,
	/// How many sats we can receive over lightning.
	pub inbound_capacity_lightning_sats: u64,
}

/// The status of your node. Useful for debugging issues.
pub struct Status {
	/// The lightning node id.
	pub node_id: String,
	/// Have a peer connection with cequals.
	pub connected: bool,
	/// Have a usable channel with cequals.
	pub usable_channels: bool,
	/// Latest synced block height.
	pub best_block_height: u32,
	/// Latest synced block hash.
	pub best_block_hash: BlockHash,
	/// Timestamp we last synced the lightning wallet.
	pub latest_wallet_sync_timestamp: Option<u64>,
	/// Timestamp we last synced the onchain wallet.
	pub latest_onchain_wallet_sync_timestamp: Option<u64>,
	/// Timestamp we last updated the fee rate cache.
	pub latest_fee_rate_cache_update_timestamp: Option<u64>,
	/// Timestamp we last updated our graph using RGS.
	pub latest_rgs_snapshot_timestamp: Option<u64>,
}

#[derive(Clone)]
/// The main interface to the lightning network
pub struct Romer {
	network: Network,
	services: ServiceConfig,
	node: Arc<Node>,
	pending_payments:
		Arc<Mutex<HashMap<PaymentHash, mpsc::SyncSender<Result<u64, PaymentFailureReason>>>>>,
}

impl Romer {
	/// Create a new Romer instance using a c= API token.
	pub fn new(api_token: &str) -> Result<Self, Error> {
		let network = token::network_from_token(api_token)?;
		let services = ServiceConfig::new(network)?;

		let services_builder = services.clone();
		let mut builder = Builder::new();
		builder.set_network(network);
		builder.set_esplora_server(services_builder.esplora_url);
		builder.set_gossip_source_rgs(services_builder.rgs_url);
		builder.set_liquidity_source_lsps2(
			services_builder.lsp_ip_port,
			services_builder.lsp_node_id,
			Some(api_token.to_string()),
		);

		let node = builder.build()?;

		node.start()?;

		let romer = Romer {
			network,
			services,
			node: Arc::new(node),
			pending_payments: Arc::new(Mutex::new(HashMap::new())),
		};

		let romer_events = romer.clone();
		std::thread::spawn(move || {
			romer_events.handle_events();
		});

		Ok(romer)
	}

	/// Receive bitcoin over the lightning network by creating an invoice to be paid a specific amount.
	///
	/// Will automatically determine if you need more liquidity and provide either a JIT-channel invoice
	/// or a regular bolt11 invoice as needed.
	pub fn receive(&self, amount_sats: u64, description: &str) -> Result<Bolt11Invoice, Error> {
		let amount_msat = amount_sats * 1000;

		let inbound_liquidity_msat: u64 = self
			.node
			.list_channels()
			.into_iter()
			.map(|channel| channel.inbound_capacity_msat)
			.sum();

		// TODO: probably some kind of factor required instead of straight comparison
		let invoice = if inbound_liquidity_msat > amount_msat {
			self.node.bolt11_payment().receive(
				amount_msat,
				description,
				DEFAULT_INVOICE_EXPIRY_SECS,
			)?
		} else {
			self.node.bolt11_payment().receive_via_jit_channel(
				amount_msat,
				description,
				DEFAULT_INVOICE_EXPIRY_SECS,
				None,
			)?
		};

		Ok(invoice)
	}

    /// Check if an invoice has been paid
    pub fn invoice_paid(&self, invoice: &Bolt11Invoice) -> bool {
        let payment_hash = PaymentHash(invoice.payment_hash().to_byte_array());
		let id = PaymentId(payment_hash.0);
        self.node.payment(&id).map_or(false, |payment| {
            matches!(payment.status, ldk_node::payment::PaymentStatus::Succeeded)
        })
    }

	/// Send bitcoin over the lightning network by paying an invoice.
	///
	/// Returns the fee paid in millisatoshis in order to complete the payment.
	pub fn send(&self, invoice: &str) -> Result<u64, Error> {
		let invoice = Bolt11Invoice::from_str(invoice).map_err(|_e| Error::InvalidBolt11Invoice)?;
		let payment_hash = PaymentHash(invoice.payment_hash().to_byte_array());
		let (sender, receiver) = mpsc::sync_channel::<Result<u64, PaymentFailureReason>>(1);
		{
			let mut pending_payments = self.pending_payments.lock().unwrap();
			pending_payments.insert(payment_hash, sender);
		}
		let _payment_id = self.node.bolt11_payment().send(&invoice)?;
		let fee_paid_msat = receiver.recv().unwrap()?;
		Ok(fee_paid_msat)
	}

	/// Send bitcoin onchain to an address.
	pub fn send_onchain(&self, address: &str, amount_sats: u64) -> Result<Txid, Error> {
		let address = Address::from_str(address).map_err(|_e| Error::InvalidBitcoinAddress)?;

		if !address.is_valid_for_network(self.network) {
			return Err(Error::InvalidBitcoinAddress);
		}

		let address = address.assume_checked();

		let txid = self.node.onchain_payment().send_to_address(&address, amount_sats)?;
		Ok(txid)
	}

	/// List all payments send or received.
	pub fn list_payments(&self) -> Vec<PaymentDetails> {
		self.node.list_payments()
	}

	/// Get balance information.
	pub fn balance(&self) -> Balances {
		let balance_details = self.node.list_balances();
		let channels = self.node.list_channels();

		let outbound_capacity_lightning_sats: u64 =
			channels.iter().map(|channel| channel.outbound_capacity_msat / 1000).sum();
		let inbound_capacity_lightning_sats: u64 =
			channels.iter().map(|channel| channel.inbound_capacity_msat / 1000).sum();

		// TODO: need to take into account min/max htlc sizes.
		//       should maybe add the "maybe claimable" portion of the lightning balance
		//       such a mess, what do we want to do here?

		Balances {
			total_onchain_balance_sats: balance_details.total_onchain_balance_sats,
			spendable_onchain_balance_sats: balance_details.spendable_onchain_balance_sats,
			total_anchor_channels_reserve_sats: balance_details.total_anchor_channels_reserve_sats,
			total_lightning_balance_sats: balance_details.total_lightning_balance_sats,
			outbound_capacity_lightning_sats,
			inbound_capacity_lightning_sats,
		}
	}

	/// Get status about the node.
	///
	/// Useful for debugging payment issues.
	pub fn status(&self) -> Status {
		let ldk_status = self.node.status();
		let usable_channels = self.node.list_channels().into_iter().find(|c| c.is_usable).is_some();
		let connected = self
			.node
			.list_peers()
			.into_iter()
			.find(|p| p.is_connected && p.node_id == self.services.lsp_node_id)
			.is_some();

		Status {
			node_id: self.node.node_id().to_string(),
			connected,
			usable_channels,
			best_block_height: ldk_status.current_best_block.height,
			best_block_hash: ldk_status.current_best_block.block_hash,
			latest_wallet_sync_timestamp: ldk_status.latest_wallet_sync_timestamp,
			latest_onchain_wallet_sync_timestamp: ldk_status.latest_onchain_wallet_sync_timestamp,
			latest_fee_rate_cache_update_timestamp: ldk_status
				.latest_fee_rate_cache_update_timestamp,
			latest_rgs_snapshot_timestamp: ldk_status.latest_rgs_snapshot_timestamp,
		}
	}

	fn handle_events(&self) {
		loop {
			let event = self.node.wait_next_event();

			match event {
				ldk_node::Event::PaymentSuccessful { payment_hash, fee_paid_msat, .. } => {
					let maybe_pending_payment = {
						let mut pending_payments = self.pending_payments.lock().unwrap();
						pending_payments.remove(&payment_hash)
					};

					if let Some(pending_payment) = maybe_pending_payment {
						if let Err(_e) = pending_payment.send(Ok(fee_paid_msat.unwrap())) {
							// TODO: log?
						}
					}
				},
				ldk_node::Event::PaymentFailed { payment_hash, reason, .. } => {
					let maybe_pending_payment = {
						let mut pending_payments = self.pending_payments.lock().unwrap();
						pending_payments.remove(&payment_hash)
					};

					if let Some(pending_payment) = maybe_pending_payment {
						if let Err(_e) = pending_payment.send(Err(reason.unwrap())) {
							// TODO: log?
						}
					}
				},
                _ => {},
			}

			self.node.event_handled();
		}
	}
}
