use tokio_util::sync::CancellationToken;

#[derive(Default, Clone)]
pub struct AppState {
    pub cancellation_token: Option<CancellationToken>,
}
