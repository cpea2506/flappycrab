mod background;
mod base;
mod bird;
mod gameover;
mod pipe;
mod score;
mod start_message;

use background::Background;
use base::Base;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use bird::Bird;
use start_message::StartMessage;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Ready,
    Playing,
    _Over,
}

struct StartMessageData {
    entity: Entity,
}

pub const _FPS: f32 = 1. / 60.;

fn main() {
    let mut app = App::new();
    let default_window = WindowDescriptor {
        title: "Flappy Crab".to_string(),
        width: 288.,
        height: 512.,
        ..default()
    };

    app.insert_resource(default_window)
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        .add_startup_system(Base::startup_system)
        .add_startup_system(Background::startup_system)
        .add_state(GameState::Ready)
        .add_system_set(SystemSet::on_update(GameState::Ready).with_system(keyboard_input_system))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(Base::moving_system)
                .with_system(Background::moving_system),
        )
        .add_system(exit_on_esc_system)
        .run();
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // message
    let start_message = commands
        .spawn_bundle(SpriteBundle {
            texture: StartMessage::texture(&asset_server),
            transform: Transform::from_xyz(0., 73.5, 0.1),
            ..default()
        })
        .id();

    commands.insert_resource(StartMessageData {
        entity: start_message,
    });

    // bird
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("images/redbird-midflap.png"),
            transform: Transform::from_xyz(-53., 9., 0.1),
            ..default()
        })
        .insert(Bird::default());
}

fn keyboard_input_system(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    start_message: Res<StartMessageData>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        commands.entity(start_message.entity).despawn();

        info!("Have remove message!");

        game_state.set(GameState::Playing).unwrap();
    }
}
