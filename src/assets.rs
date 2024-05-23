use bevy::prelude::*;
use bevy::sprite::{
    Mesh2dHandle,
    MaterialMesh2dBundle
};

pub struct MenuInitPlugin;
impl Plugin for MenuInitPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, Self::initialize_menu);
    }
}

impl MenuInitPlugin {
    fn initialize_menu(
        mut command: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let start_button_mesh = Mesh2dHandle(meshes.add(Capsule2d::new(300.0, 100.0)));
        let start_button_color = Color::hsl(0.0, 0.95, 0.7);

        command.spawn(MaterialMesh2dBundle {
            mesh: start_button_mesh,
            material: materials.add(start_button_color),
            transform: Transform::from_xyz(0.0, -270.0, 0.0),
            ..default()
        });
    }
}
