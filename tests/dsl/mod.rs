use cucumber_rust::{async_trait, World, WorldInit};
use std::convert::Infallible;

#[derive(WorldInit)]
pub struct ScenarioEnvironment {}

#[async_trait(?Send)]
impl World for ScenarioEnvironment {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
