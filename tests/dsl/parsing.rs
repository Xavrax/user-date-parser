use cucumber_rust::{given, then, when};
use user_date_parser::UserDateParser;

use crate::dsl::ScenarioEnvironment;

// todo: consider using mockall crate
//       https://docs.rs/mockall/0.10.2/mockall/
struct MockUserDateParser;

impl UserDateParser for MockUserDateParser {}

#[given("implemented parser")]
fn init_parser(env: &mut ScenarioEnvironment) {
    env.parser = Some(Box::new(MockUserDateParser));
}
