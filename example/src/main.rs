use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_cursor_hovering_sprite::*;

#[derive(Component)]
struct SpriteName{
    name: String,
}

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CursorHoveringSpritePlugin)
    .add_systems(Startup, setup)
    .add_systems(Update, checking)
    .add_systems(Update, toggle_state)
    .run();
}

fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut border_asset: ResMut<Assets<BorderPolygon>>,
    mut cursor_hovering_camera: ResMut<CursorHoveringCamera>,
){
    let window = window_query.get_single().unwrap();

    let camera_entity = commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    ).id();
    cursor_hovering_camera.entity = Some(camera_entity); // Register the checking camera

    // create a border asset
    let border_polygon = border_asset.add(
        BorderPolygon{
            points: vec![
                Vec2{x:0.0, y:100.0},
                Vec2{x:70.0, y:70.0},
                Vec2{x:100.0, y:0.0},
                Vec2{x:70.0, y:-70.0},
                Vec2{x:0.0, y:-70.0},
                Vec2{x:-70.0, y:-70.0},
                Vec2{x:-100.0, y:0.0},
                Vec2{x:-70.0, y:70.0},
            ]
        }
    );

    commands.spawn(
        (
        SpriteBundle{
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("white_circle.png"),
            ..default()
        },
        SpriteBorder{
            polygon: border_polygon.clone(),
        },
        SpriteName{
            name: "WHITE".to_string(),
        },
    ));

    commands.spawn(
        (
        SpriteBundle{
            transform: Transform::from_xyz(window.width() / 2.0 + 60.0, window.height() / 2.0 - 60.0, 1.0),
            texture: asset_server.load("yellow_circle.png"),
            ..default()
        },
        SpriteBorder{
            polygon: border_polygon.clone(), 
        },
        SpriteName{
            name: "YELLOW".to_string(),
        },
    ));

    commands.spawn(
        (
        SpriteBundle{
            transform: Transform::from_xyz(window.width() / 2.0 - 60.0, window.height() / 2.0 - 60.0, 2.0),
            texture: asset_server.load("blue_circle.png"),
            ..default()
        },
        SpriteBorder{
            polygon: border_polygon.clone(), 
        },
        SpriteName{
            name: "BLUE".to_string(),
        },
    ));
}

fn checking(
    mut event_reader: EventReader<CursorOnSprite>,
    sprite_query: Query<&SpriteName>
){
    if event_reader.is_empty(){
        return
    }
    for event in event_reader.read(){
        let sprite_entity = event.entity;
        let sprite_name = sprite_query.get(sprite_entity).unwrap();
        println!("On entity:{}", sprite_name.name);
    }
}

fn toggle_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<CursorHoveringSpriteState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match **state{
            CursorHoveringSpriteState::Checking => {
                commands.insert_resource(NextState(Some(CursorHoveringSpriteState::Idling)));
                println!("Hover checking stopped!");
            }
            CursorHoveringSpriteState::Idling => {
                commands.insert_resource(NextState(Some(CursorHoveringSpriteState::Checking)));
                println!("Hover checking starts!")
            }
        }
    }
}