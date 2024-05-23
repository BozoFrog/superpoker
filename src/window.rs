use bevy::{
    prelude::*,
    window::WindowResolution,
};

pub struct WindowInitPlugin;
impl Plugin for WindowInitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Self::initialize_window)
            .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)));
    }
}

impl WindowInitPlugin {
    fn initialize_window(
        mut windows: Query<&mut Window>,
        mut command: Commands,
    ) {
        command.spawn(Camera2dBundle::default());
        let mut window = windows.single_mut();
        window.resolution = WindowResolution::new(1920.0, 1080.0);
    }
}
