# LDK Node

[![Crate](https://img.shields.io/crates/v/romer.svg?logo=rust)](https://crates.io/crates/romer)
[![Documentation](https://img.shields.io/static/v1?logo=read-the-docs&label=docs.rs&message=romer&color=informational)](https://docs.rs/romer)
[![Maven Central Android](https://img.shields.io/maven-central/v/org.cequals/romer-android)](https://central.sonatype.com/artifact/org.cequals/romer-android)
[![Maven Central JVM](https://img.shields.io/maven-central/v/org.cequals/romer-jvm)](https://central.sonatype.com/artifact/org.cequals/romer-jvm)

A ready-to-go library for integrating lightning payments into your application.

Romer is a non-custodial Lightning node in library form. Its central goal is to provide a
small, simple, and straightforward interface that enables users to easily send and receive
payments over the Lightning network.

## Getting Started

The primary abstraction of the library is [`Romer`][api_docs_romer], which can be constructed by providing
your c= api key to `Rome::new`. `Romer` can then be controlled via commands such as `send`, `receive`, `list_payments`,
`balance`, etc.

```rust
use romer::Romer;
fn main() {
	let romer = Romer::new("my-api-key").unwrap();

    // receive bitcoin by creating an invoice
    let invoice = romer.receive(100_000, "alpaca socks").unwrap();
    
    // send bitcoin by paying a lightning invoice
	romer.send("INVOICE_STR").unwrap();

    // see all payments sent and received
    let payments = romer.list_payments();

    // see balance information
    let balance = romer.balance();
}
```

## Language Support
Romer itself is written in [Rust][rust] and may therefore be natively added as a library dependency to any `std` Rust program. However, beyond its Rust API it also offers language bindings for [Swift][swift], [Kotlin][kotlin], and [Python][python] based on the [UniFFI](https://github.com/mozilla/uniffi-rs/).

## MSRV
The Minimum Supported Rust Version (MSRV) is currently 1.63.0.

[api_docs]: https://docs.rs/romer/*/romer/
[api_docs_node]: https://docs.rs/romer/*/romer/struct.Node.html
[api_docs_builder]: https://docs.rs/romer/*/romer/struct.Builder.html
[rust]: https://www.rust-lang.org/
[swift]: https://www.swift.org/
[kotlin]: https://kotlinlang.org/
[python]: https://www.python.org/