#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait Foo: Send + Sync {
    async fn Foo(&self);
}

uniffi::setup_scaffolding!();
