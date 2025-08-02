//! Value estimation implementations.

mod torch_value;
mod value_estimator;

pub use torch_value::TorchValueEstimator;
pub use value_estimator::ValueEstimator;
