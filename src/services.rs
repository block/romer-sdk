// This file is Copyright its original authors, visible in version contror
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.
use std::str::FromStr;

use ldk_node::{
	bitcoin::{secp256k1::PublicKey, Network},
	lightning::ln::msgs::SocketAddress,
};

use crate::error::Error;

const SIGNET_ESPLORA_URL: &str = "https://staging.e.r.cequals.xyz";
const SIGNET_RGS_URL: &str = "https://staging.r.r.cequals.xyz";
const SIGNET_LSP_NODE_ID: &str =
	"0371d6fd7d75de2d0372d03ea00e8bacdacb50c27d0eaea0a76a0622eff1f5ef2b";
const SIGNET_LSP_IP_PORT: SocketAddress =
	SocketAddress::TcpIpV4 { addr: [44, 219, 111, 31], port: 39735 };

const MAINNET_ESPLORA_URL: &str = "https://e.r.cequals.xyz";
const MAINNET_RGS_URL: &str = "https://r.r.cequals.xyz";
const MAINNET_LSP_NODE_ID: &str =
	"027100442c3b79f606f80f322d98d499eefcb060599efc5d4ecb00209c2cb54190";
const MAINNET_LSP_IP_PORT: SocketAddress =
	SocketAddress::TcpIpV4 { addr: [3, 226, 165, 222], port: 9735 };

#[derive(Clone)]
pub struct ServiceConfig {
	pub esplora_url: String,
	pub rgs_url: String,
	pub lsp_node_id: PublicKey,
	pub lsp_ip_port: SocketAddress,
}

impl ServiceConfig {
	pub fn new(network: Network) -> Result<Self, Error> {
		match network {
			Network::Signet => Ok(Self {
				esplora_url: SIGNET_ESPLORA_URL.to_string(),
				rgs_url: SIGNET_RGS_URL.to_string(),
				lsp_node_id: PublicKey::from_str(SIGNET_LSP_NODE_ID).expect("valid public key"),
				lsp_ip_port: SIGNET_LSP_IP_PORT,
			}),
			Network::Bitcoin => Ok(Self {
				esplora_url: MAINNET_ESPLORA_URL.to_string(),
				rgs_url: MAINNET_RGS_URL.to_string(),
				lsp_node_id: PublicKey::from_str(MAINNET_LSP_NODE_ID).expect("valid public key"),
				lsp_ip_port: MAINNET_LSP_IP_PORT,
			}),
			_ => return Err(Error::NetworkNotSupported),
		}
	}
}
