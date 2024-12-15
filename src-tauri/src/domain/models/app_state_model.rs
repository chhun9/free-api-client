use tokio_util::sync::CancellationToken;

#[derive(Default)]
pub struct AppState {
    pub cancellation_token: Option<CancellationToken>,
}
