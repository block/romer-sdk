// This file is Copyright its original authors, visible in version contror
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.
use std::fmt;

use ldk_node::lightning::events::PaymentFailureReason;

#[derive(Debug, PartialEq, Eq)]
/// An error that possibly needs to be handled by the user.
pub enum Error {
	/// Invalid API Token provided
	InvalidAPIToken,
	/// Network not supported
	NetworkNotSupported,
	/// Invalid Bolt11 invoice
	InvalidBolt11Invoice,
	/// Invalid bitcoin address
	InvalidBitcoinAddress,
	/// Invalid offer id
	InvalidOfferId,
	/// Invalid payment id
	InvalidPaymentId,
	/// Invalid payment hash
	InvalidPaymentHash,
	/// Invalid payment preimage
	InvalidPaymentPreimage,
	/// Invalid payment secret
	InvalidPaymentSecret,
	/// Failed to build node
	FailedToBuildNode,
	/// Internal LDK Node error
	LDKNodeError,
	/// The intended recipient rejected our payment.
	RecipientRejected,
	/// We exhausted all of our retry attempts while trying to send the payment.
	RetriesExhausted,
	/// The payment expired while retrying.
	PaymentExpired,
	/// We failed to find a route while retrying the payment.
	///
	/// Note that this generally indicates that we've exhausted the available set of possible
	/// routes - we tried the payment over a few routes but were not able to find any further
	/// candidate routes beyond those.
	RouteNotFound,
	/// Something unexpected happened.
	UnexpectedError,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::InvalidAPIToken => write!(f, "Invalid API token provided."),
			Self::NetworkNotSupported => write!(f, "Network not supported."),
			Self::InvalidBolt11Invoice => write!(f, "Invalid Bolt11 invoice provided."),
			Self::InvalidBitcoinAddress => write!(f, "Invalid bitcoin address provided."),
			Self::InvalidOfferId => write!(f, "Invalid offer id."),
			Self::InvalidPaymentId => write!(f, "Invalid payment id."),
			Self::InvalidPaymentHash => write!(f, "Invalid payment hash provided."),
			Self::InvalidPaymentPreimage => write!(f, "Invalid payment preimage provided."),
			Self::InvalidPaymentSecret => write!(f, "Invalid payment secret provided."),
			Self::FailedToBuildNode => write!(f, "Failed to build node"),
			Self::LDKNodeError => write!(f, "Internal LDK Node error."),
			Self::RecipientRejected => write!(f, "The intendfed recipient rejected the payment."),
			Self::RetriesExhausted => write!(
				f,
				"We exhausted all of our retry attempts while trying to send the payment."
			),
			Self::PaymentExpired => write!(f, "The payment expired while retrying."),
			Self::RouteNotFound => {
				write!(f, "We failed to find a route while retrying the payment.")
			},
			Self::UnexpectedError => write!(f, "Something unexpected happened."),
		}
	}
}

impl std::error::Error for Error {}

impl From<ldk_node::BuildError> for Error {
	fn from(_value: ldk_node::BuildError) -> Self {
		Error::FailedToBuildNode
	}
}

impl From<ldk_node::NodeError> for Error {
	fn from(_value: ldk_node::NodeError) -> Self {
		Error::LDKNodeError
	}
}

impl From<PaymentFailureReason> for Error {
	fn from(value: PaymentFailureReason) -> Self {
		match value {
			PaymentFailureReason::RecipientRejected => Error::RecipientRejected,
			PaymentFailureReason::UserAbandoned => Error::UnexpectedError,
			PaymentFailureReason::RetriesExhausted => Error::RetriesExhausted,
			PaymentFailureReason::PaymentExpired => Error::PaymentExpired,
			PaymentFailureReason::RouteNotFound => Error::RouteNotFound,
			PaymentFailureReason::UnexpectedError => Error::UnexpectedError,
		}
	}
}
