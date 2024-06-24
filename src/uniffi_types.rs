use std::fmt::Write;
use std::str::FromStr;

use ldk_node::bitcoin::hashes::sha256::Hash as Sha256;
use ldk_node::bitcoin::hashes::Hash;
pub use ldk_node::bitcoin::{Address, BlockHash, Txid};
use ldk_node::lightning::ln::channelmanager::PaymentId;
pub use ldk_node::lightning::ln::{PaymentHash, PaymentPreimage, PaymentSecret};
pub use ldk_node::lightning::offers::offer::OfferId;
pub use ldk_node::lightning_invoice::Bolt11Invoice;
use ldk_node::lightning_invoice::SignedRawBolt11Invoice;
pub use ldk_node::payment::{LSPFeeLimits, PaymentDirection, PaymentKind, PaymentStatus};

use crate::error::Error;
use crate::UniffiCustomTypeConverter;

pub fn to_vec(hex: &str) -> Option<Vec<u8>> {
	let mut out = Vec::with_capacity(hex.len() / 2);

	let mut b = 0;
	for (idx, c) in hex.as_bytes().iter().enumerate() {
		b <<= 4;
		match *c {
			b'A'..=b'F' => b |= c - b'A' + 10,
			b'a'..=b'f' => b |= c - b'a' + 10,
			b'0'..=b'9' => b |= c - b'0',
			_ => return None,
		}
		if (idx & 1) == 1 {
			out.push(b);
			b = 0;
		}
	}

	Some(out)
}

#[inline]
pub fn to_string(value: &[u8]) -> String {
	let mut res = String::with_capacity(2 * value.len());
	for v in value {
		write!(&mut res, "{:02x}", v).expect("Unable to write");
	}
	res
}

impl UniffiCustomTypeConverter for Address {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(addr) = Address::from_str(&val) {
			return Ok(addr.assume_checked());
		}

		Err(Error::InvalidBitcoinAddress.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}

impl UniffiCustomTypeConverter for Bolt11Invoice {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(signed) = val.parse::<SignedRawBolt11Invoice>() {
			if let Ok(invoice) = Bolt11Invoice::from_signed(signed) {
				return Ok(invoice);
			}
		}

		Err(Error::InvalidBolt11Invoice.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}

impl UniffiCustomTypeConverter for OfferId {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Some(bytes_vec) = to_vec(&val) {
			let bytes_res = bytes_vec.try_into();
			if let Ok(bytes) = bytes_res {
				return Ok(OfferId(bytes));
			}
		}
		Err(Error::InvalidOfferId.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		to_string(&obj.0)
	}
}

impl UniffiCustomTypeConverter for PaymentId {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Some(bytes_vec) = to_vec(&val) {
			let bytes_res = bytes_vec.try_into();
			if let Ok(bytes) = bytes_res {
				return Ok(PaymentId(bytes));
			}
		}
		Err(Error::InvalidPaymentId.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		to_string(&obj.0)
	}
}

impl UniffiCustomTypeConverter for PaymentHash {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Ok(hash) = Sha256::from_str(&val) {
			Ok(PaymentHash(hash.to_byte_array()))
		} else {
			Err(Error::InvalidPaymentHash.into())
		}
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		Sha256::from_slice(&obj.0).unwrap().to_string()
	}
}

impl UniffiCustomTypeConverter for PaymentPreimage {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Some(bytes_vec) = to_vec(&val) {
			let bytes_res = bytes_vec.try_into();
			if let Ok(bytes) = bytes_res {
				return Ok(PaymentPreimage(bytes));
			}
		}
		Err(Error::InvalidPaymentPreimage.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		to_string(&obj.0)
	}
}

impl UniffiCustomTypeConverter for PaymentSecret {
	type Builtin = String;

	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		if let Some(bytes_vec) = to_vec(&val) {
			let bytes_res = bytes_vec.try_into();
			if let Ok(bytes) = bytes_res {
				return Ok(PaymentSecret(bytes));
			}
		}
		Err(Error::InvalidPaymentSecret.into())
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		to_string(&obj.0)
	}
}

impl UniffiCustomTypeConverter for Txid {
	type Builtin = String;
	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		Ok(Txid::from_str(&val)?)
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}

impl UniffiCustomTypeConverter for BlockHash {
	type Builtin = String;
	fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
		Ok(BlockHash::from_str(&val)?)
	}

	fn from_custom(obj: Self) -> Self::Builtin {
		obj.to_string()
	}
}
