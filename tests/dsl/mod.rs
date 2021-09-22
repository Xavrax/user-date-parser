use std::convert::Infallible;

use cucumber_rust::{async_trait, World, WorldInit};
use cucumber_rust::{given, then, when};
use user_date_parser::UserDateParser;

mod parsing;

#[derive(WorldInit)]
pub struct ScenarioEnvironment {
    pub user_friendly_string: String,
    pub parser: Option<Box<dyn UserDateParser>>,
}

#[async_trait(?Send)]
impl World for ScenarioEnvironment {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            user_friendly_string: "".into(),
            parser: None,
        })
    }
}

#[given(regex = "user-friendly (.*)")]
fn set_user_friendly_string(env: &mut ScenarioEnvironment, string: String) {
    env.user_friendly_string = string;
}

#[given(regex = "relative date set as \"(.*)\"")]
fn set_relative_date(env: &mut ScenarioEnvironment, date: String) {}
