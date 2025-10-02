use tower::{ServiceBuilder, limit::ConcurrencyLimitLayer, timeout::TimeoutLayer, util::BoxCloneService};
use std::time::Duration;

pub fn resilient_layer() -> ServiceBuilder<()>{ 
  ServiceBuilder::new()
    .layer(ConcurrencyLimitLayer::new(64))
    .layer(TimeoutLayer::new(Duration::from_millis(800)))
}
