#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to bind to address")]
    Bind(#[source] std::io::Error),
    #[error("failed to listen on address {0}")]
    Listen(#[source] std::io::Error, &'static str),
    #[error("failed to serve request")]
    Serve(#[source] std::io::Error),
}
