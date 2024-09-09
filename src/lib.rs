pub mod constants;
pub mod tokio;
pub mod torii;

use bevy::prelude::*;
use tokio::TokioPlugin;

pub struct DojoPlugin;
impl Plugin for DojoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dojo_startup);
        app.add_plugins(TokioPlugin);
    }
}

fn dojo_startup() {
    info!("Dojo Plugin is starting up!");
}
