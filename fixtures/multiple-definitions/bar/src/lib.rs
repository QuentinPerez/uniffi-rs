#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait Bar: Send + Sync {
    async fn bar(&self);
}


uniffi::setup_scaffolding!();
