use bevy::prelude::*;

pub struct DojoPlugin;
impl Plugin for DojoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, dojo_startup);
    }
}

fn dojo_startup() {
    info!("Dojo Plugin is starting up!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn import_plugin() {
        App::new().add_plugins(DojoPlugin).run();
    }
}
