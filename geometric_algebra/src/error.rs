use thiserror::Error;

#[derive(Debug, Error)]
pub enum GAError {
    #[error("trying to create point from infinite point")]
    PointFromInfinitePoint,
}
