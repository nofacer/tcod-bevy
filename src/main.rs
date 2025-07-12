use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player {
    timer: Timer,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture_handle = asset_server.load("images/dejavu16x16_gs_tc.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 32, 8, None, None);
    let layout_handle = texture_atlases.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(
            texture_handle,
            TextureAtlas {
                layout: layout_handle,
                index: 32,
            },
        ),
        Transform::from_xyz(0., 0., 0.),
        Player {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Single<(&mut Player, &mut Transform)>,
) {
    let (mut _player, mut transform) = query.into_inner();
    _player.timer.tick(time.delta());
    if keyboard_input.pressed(KeyCode::ArrowRight) && _player.timer.just_finished() {
        transform.translation.x += 16.0;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) && _player.timer.just_finished() {
        transform.translation.x -= 16.0;
    } else if keyboard_input.pressed(KeyCode::ArrowUp) && _player.timer.just_finished() {
        transform.translation.y += 16.0;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) && _player.timer.just_finished() {
        transform.translation.y -= 16.0;
    }
}
