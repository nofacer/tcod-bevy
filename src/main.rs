use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
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

    commands.spawn(Sprite::from_atlas_image(
        texture_handle,
        TextureAtlas {
            layout: layout_handle,
            index: 32,
        },
    ));
}
