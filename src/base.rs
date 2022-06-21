use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Base {
    pub x: f32,
    pub secondary: bool,
}

impl Base {
    pub fn startup_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window: Res<Windows>,
    ) {
        let window = window.get_primary().unwrap();
        let texture = asset_server.load("images/base.png");

        commands
            .spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                transform: Transform::from_xyz(0., 56. - window.height() / 2., 0.1),
                ..default()
            })
            .insert(Base::default());
        commands
            .spawn_bundle(SpriteBundle {
                texture,
                transform: Transform::from_xyz(168., 56. - window.height() / 2.0, 0.1),
                ..default()
            })
            .insert(Base {
                x: 168.,
                secondary: true,
            });
    }

    pub fn moving_system(mut query: Query<(&mut Base, &mut Transform)>) {
        for (mut base, mut transform) in query.iter_mut() {
            base.x = (base.x - 1.) % 168.;

            if base.secondary {
                transform.translation.x = base.x + 168.;
            } else {
                transform.translation.x = base.x;
            }
        }
    }
}
