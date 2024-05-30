# Example for the cursor_hovering_sprite plugin 

## What does this example show?

This is a minimum bevy project for showing the usage of the plugin.

In this code, I use a octagon to present the border for a circle sprite.

I put three circles overlapping each other with different z values.

When hovering the cursor on the circle, it will print a message in the terminal reporting which circle is under the cursor.
If the cursor is on the overlapping region, the top one with the larges z value will be reported.

## How to use the plugin
 
First register the plugin to your bevy app.

```rust
fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CursorHoveringSpritePlugin) // register the plguin
    /*...*/
    .run();
}
```

Then create a BorderPolygon asset handle.
For example, I created a octagon here:

```rust
//mut border_asset: ResMut<Assets<BorderPolygon>>

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
```

If you want a sprite to be checked hovering on later, attach a SpriteBorder component wrapping this handle when you spawn this sprite entity.
For example:

```rust
//mut commands: Commands

 commands.spawn(
        (
        SpriteBundle{
            texture: asset_server.load("white_circle.png"),
            ..default()
        },
        SpriteBorder{
            polygon: border_polygon.clone(),
        },
    ));
```

After doing this, you can check if this entity is under the cursor by reading the event __CursorOnSprite__.
For example:

```rust
// mut event_reader: EventReader<CursorOnSprite>

for event in event_reader.read(){
    //Here, event.entity stores the entity currently under the cursor
}
```

You can pause the checking by switching the CursorHoveringSpriteState to **Idling**.
The CursorOnSprite event only emits in CursorHoveringSpriteState::Checking state.

Have fun!