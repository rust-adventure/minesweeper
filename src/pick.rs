use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Event)]
pub struct RevealTile {
    pub is_primary_click: bool,
    pub tile_pos: TilePos,
}

pub fn pick_tile(
    mut commands: Commands,
    window: Single<&Window>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TileStorage,
        &Transform,
        &TilemapAnchor,
    )>,
    camera: Single<(&GlobalTransform, &Camera)>,
    mut tiles: Query<&mut TileTextureIndex>,
    input: Res<ButtonInput<MouseButton>>,
) {
    for (
        map_size,
        grid_size,
        tile_size,
        map_type,
        tile_storage,
        map_transform,
        anchor,
    ) in tilemap_q.iter()
    {
        let Some(cursor_pos_in_window) =
            window.cursor_position()
        else {
            return;
        };
        let Ok(cursor_pos) = camera.1.viewport_to_world_2d(
            camera.0,
            cursor_pos_in_window,
        ) else {
            return;
        };

        // We need to make sure that the cursor's world
        // position is correct relative to the map
        // due to any map transformation.
        let cursor_in_map_pos: Vec2 = {
            // Extend the cursor_pos vec3 by 0.0 and 1.0
            let cursor_pos =
                Vec4::from((cursor_pos, 0.0, 1.0));
            let cursor_in_map_pos =
                map_transform.compute_matrix().inverse()
                    * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform
        // it into a possible tile position.
        if let Some(tile_pos) = TilePos::from_world_pos(
            &cursor_in_map_pos,
            map_size,
            grid_size,
            tile_size,
            map_type,
            anchor,
        ) {
            if let Some(tile_entity) =
                tile_storage.get(&tile_pos)
            {
                if input.just_pressed(MouseButton::Right) {
                    match tiles.get(tile_entity).unwrap().0
                    {
                        14 => {
                            tiles
                                .get_mut(tile_entity)
                                .unwrap()
                                .0 = 10
                        }
                        10 => {
                            tiles
                                .get_mut(tile_entity)
                                .unwrap()
                                .0 = 14
                        }
                        _ => {}
                    }
                } else {
                    commands.trigger_targets(
                        RevealTile {
                            is_primary_click: true,
                            tile_pos,
                        },
                        tile_entity,
                    )
                }
            }
        }
    }
}
