use std::sync::Arc;

#[uniffi::export]
pub fn hello(foo: Arc<dyn foo::Foo>, bar: Arc<dyn bar::Bar>) {}

uniffi::setup_scaffolding!();
