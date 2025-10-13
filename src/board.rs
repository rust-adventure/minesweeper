use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

use crate::assets::ImageAssets;

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct Concealed;

pub fn spawn_board(
    mut commands: Commands,
    images: Res<ImageAssets>,
) -> Result {
    let mut rng = rand::rng();

    let map_size = TilemapSize { x: 20, y: 20 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(
                            tilemap_entity,
                        ),
                        texture_index: TileTextureIndex(10),
                        ..default()
                    },
                    Concealed,
                ))
                .id();
            let make_entity_bomb =
                rng.random::<f32>() > 0.9;
            if make_entity_bomb {
                commands.entity(tile_entity).insert(Bomb);
            }
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 64., y: 64. };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(
            images.tiles.clone(),
        ),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..default()
    });

    Ok(())
}
