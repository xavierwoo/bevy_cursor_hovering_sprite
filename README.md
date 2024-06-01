# Cursor hovering sprite checker plugin for bevy 

Here is a very lightweight plugin for bevy engine to check if a cursor is hovering on one 2d sprite.

| bevy  | bevy_cursor_hovering_sprite |
|-------|---------------------|
| 0.13  | 0.1.0                 |


## What does it do?

When adding this plugin to your bevy project and attaching a **SpriteBorder** component to one entity (may be a sprite), this plugin checks if the cursor is currently hovering in the region defined by the SpriteBorder and the entity's coordinate.
If the cursor is hovering on one entity, the plugin emits an event **CursorOnSprite**, reporting the entity.

It can be used to implement a 2d sprite picking funtionality.

Check the _example_ folder to see how to use it.

## What is the idea behind it?

The idea is simple.
First translate the cursor position into the 2d world coordinate.
And then check whether this coordinate resides in the polygon calculated from the **SpriteBorder** component and the entity's world coordinate.

I drew lessons from 
https://www.geeksforgeeks.org/how-to-check-if-a-given-point-lies-inside-a-polygon/
to check whether a point is inside a polygon.

To save the energy, only entities visible in the camera is checked by iterating through **VisibleEntities**.

## What does this plugin add into your project?

When adding this plugin, the following things will be added to your project:

1. A Resource to mark which camera is used for checking. Yes, only one camera is allowed. I am sorry if you need multiple cameras checking at the same time.

```rust
#[derive(Resource)]
pub struct CursorHoveringCamera{
    pub entity: Option<Entity>,
}
```

2. An Asset to store the border polygon data. The polygon is defined by a serials of vertices.

```rust
#[derive(Asset, TypePath, Debug)]
pub struct BorderPolygon{
    pub points: Vec<Vec2>,
}
```

3. A Component type wrapping the BorderPolygon handle. You should attach this component to entities that you want to be checked.

```rust
#[derive(Component)]
pub struct SpriteBorder{
    pub polygon: Handle<BorderPolygon>,
}
```

4. An Event type. When the plugin finds an entity under the cursor, a CursorOnSprite event holding the entity is emited.

```rust
#[derive(Event)]
pub struct CursorOnSprite{
    pub entity: Entity,
}
```

5. A State that marks whether the checking functions right now. You can change the state to _Idling_ to pause the checking.

```rust
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CursorHoveringSpriteState{
    #[default]
    Checking,
    Idling,
}
```

6. An Update system. This system is responsible for checking the entities under the cursor. It runs if CursorHoveringSpriteState is in _Checking_.

```rust
fn cursor_hovering(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    visible_entities_query: Query<&VisibleEntities>,
    sprite_query: Query<(&Transform, &SpriteBorder)>,
    border_asset: Res<Assets<BorderPolygon>>,
    picking_camera: Res<CursorHoveringCamera>, 
    mut cursor_on_event_writer: EventWriter<CursorOnSprite>,
){/*...*/}
```