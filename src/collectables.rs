use bevy::prelude::*;

#[derive(Component)]
pub enum Collectable {
    Scroll,
}

fn spawn_collectables(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = asset_server.load("scroll.png");
    commands.spawn((
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_xyz(8., 8., 10.),
            ..default()
        },
        Collectable::Scroll,
    ));
}

pub struct CollectablesPlugin;
impl Plugin for CollectablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_collectables);
    }
}

