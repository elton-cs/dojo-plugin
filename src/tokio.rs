use bevy::prelude::*;
use tokio::runtime::Runtime;

pub struct TokioPlugin;
impl Plugin for TokioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tokio_client);
    }
}

#[derive(Resource)]
pub struct TokioRuntime {
    pub runtime: Runtime,
}

fn setup_tokio_client(mut commands: Commands) {
    let runtime = Runtime::new().unwrap();
    commands.insert_resource(TokioRuntime { runtime });
}
