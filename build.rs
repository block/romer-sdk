fn main() {
	#[cfg(feature = "uniffi")]
	uniffi::generate_scaffolding("bindings/romer.udl").unwrap();
}
