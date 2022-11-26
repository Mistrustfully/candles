use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
            .add_startup_system(setup);
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}
