use bevy::prelude::*;
use bevy::render::view::VisibleEntities;

pub struct CursorHoveringSpritePlugin;


impl Plugin for CursorHoveringSpritePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<CursorHoveringCamera>()
        .init_asset::<BorderPolygon>()
        .init_state::<CursorHoveringSpriteState>()
        .add_event::<CursorOnSprite>()
        .add_systems(Update, cursor_hovering.run_if(in_state(CursorHoveringSpriteState::Checking)))
        ;
    }
}

#[derive(Resource)]
pub struct CursorHoveringCamera{
    pub entity: Option<Entity>,
}

impl Default for CursorHoveringCamera {
    fn default() -> CursorHoveringCamera {
        CursorHoveringCamera{
            entity: None,
        }
    }
}


#[derive(Asset, TypePath, Debug)]
pub struct BorderPolygon{
    pub points: Vec<Vec2>,
}

#[derive(Component)]
pub struct SpriteBorder{
    pub polygon: Handle<BorderPolygon>,
}

#[derive(Event)]
pub struct CursorOnSprite{
    pub entity: Entity,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CursorHoveringSpriteState{
    #[default]
    Checking,
    Idling,
}

fn get_cursor_xy(
    window_query: &Query<&Window>,
    camera_query: &Query<(&Camera, &GlobalTransform)>,
    camera_entity: Entity,
) -> Option<Vec2> {
    let window = window_query.single();
    let cursor_pos = window.cursor_position();
    if cursor_pos == None {return None}
    match cursor_pos{
        None => {return None}
        Some(..) => {
            if let Ok((camera, camera_transform)) = camera_query.get(camera_entity){
                return cursor_pos
                            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            } else {
                return None
            }
        }
    }
}

fn point_is_inside_polygon(
    point_pos: Vec2,
    polygon_pos: Vec2,
    polygon_border: &Vec<Vec2>,
) -> bool {
    let num_vertices = polygon_border.len();
    let mut is_inside = false;

    let x = point_pos.x;
    let y = point_pos.y;

    for i in 0..num_vertices{
        let p1 = polygon_pos + polygon_border[i];
        let p2 = polygon_pos + polygon_border[(i+1) % num_vertices];
        if y > f32::min(p1.y, p2.y) 
        && y <= f32::max(p1.y, p2.y)
        && x <= f32::max(p1.x, p2.x) {
            let x_intersection = (y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;

            if p1.x == p2.x || x <= x_intersection {
                is_inside = !is_inside;
            }
        }
    }

    is_inside
}

fn cursor_hovering(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    visible_entities_query: Query<&VisibleEntities>,
    sprite_query: Query<(&Transform, &SpriteBorder)>,
    border_asset: Res<Assets<BorderPolygon>>,
    picking_camera: Res<CursorHoveringCamera>, 
    mut cursor_on_event_writer: EventWriter<CursorOnSprite>,
){
    let mut hoving_entity: Option<Entity> = None;
    let mut hoving_entity_z = -f32::INFINITY;

    if let Some(camera_entity) = picking_camera.entity {
        if let Some(cursor_xy) = get_cursor_xy(&window_query, &camera_query, camera_entity){
            for entity in visible_entities_query.single().iter(){
                if let Ok((transform, border))= sprite_query.get(*entity){
                    
                    if transform.translation.z < hoving_entity_z {continue}

                    let border_pos = Vec2{x:transform.translation.x, y:transform.translation.y};
                    let polygon_border = border_asset.get(border.polygon.clone()).unwrap();
                    if point_is_inside_polygon(cursor_xy, border_pos, &polygon_border.points) {
                        hoving_entity = Some(*entity);
                        hoving_entity_z = transform.translation.z;
                    }
                }
            }
        }
    }

    if let Some(entity_id) = hoving_entity { 
        cursor_on_event_writer.send(CursorOnSprite{entity: entity_id});
    }
}
