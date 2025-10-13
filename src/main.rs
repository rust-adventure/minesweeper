use bevy::{
    input::common_conditions::input_just_pressed,
    camera::ScalingMode,
    prelude::*, 
};
use bevy_ecs_tilemap::{
    TilemapPlugin,
    map::TilemapSize,
    tiles::{TilePos, TileStorage, TileTextureIndex},
};
use minesweeper::{
    GameState,
    assets::AssetsPlugin,
    board::{Bomb, Concealed, spawn_board},
    pick::{RevealTile, pick_tile},
};

#[derive(Event)]
struct RevealBombs;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins((
            DefaultPlugins,
            TilemapPlugin,
        ))
        .init_state::<GameState>()
        .add_plugins(AssetsPlugin)
        .add_systems(
            OnEnter(GameState::Startup),
            (
                setup,
                spawn_board,
                transition_to_start_menu,
            )
                .chain(),
        )
        .add_systems(
            Update,
            pick_tile.run_if(input_just_pressed(
                MouseButton::Left,
            ).or(input_just_pressed(
                MouseButton::Right,
            ))),
        )
        .add_observer(
            |reveal_tile: On<RevealTile>,
            mut commands: Commands,
             bombs: Query<(), With<Bomb>>,
             map: Single<(&TilemapSize, &TileStorage)>,
             concealed_tiles: Query<&Concealed>,
             mut tiles: Query<(&TilePos, &mut TileTextureIndex)>,
             | -> Result {
                if bombs.get(reveal_tile.entity).is_ok() {
                    // if the tile is a bomb and the player clicked it directly;
                    // boom.
                    if reveal_tile.event().is_primary_click {
                        commands.trigger(RevealBombs);
                        commands.entity(reveal_tile.entity).remove::<Concealed>();
         
                    }
                } else if concealed_tiles.get(reveal_tile.entity).is_ok() {
                    // if the tile is hidden, then try to reveal it
                    commands.entity(reveal_tile.entity).remove::<Concealed>();
                    let RevealTile {
                        tile_pos,
                        ..
                    } = reveal_tile.event();

                    let (map_size, tile_storage) = map.into_inner();

                    let Ok(tile) = tiles.get(reveal_tile.entity) else {
                        return Ok(());
                    };
                    let bomb_count = [
                        IVec2::X +     IVec2::Y,
                        IVec2::X,
                        IVec2::X +     IVec2::NEG_Y,
                                       IVec2::NEG_Y,
                        IVec2::NEG_X + IVec2::NEG_Y,
                        IVec2::NEG_X,
                        IVec2::NEG_X + IVec2::Y,
                                       IVec2::Y,
                    ].into_iter().map(|offset| {
                        let upos = UVec2::from(tile.0);
                        let next_pos = TilePos::from(upos.wrapping_add_signed(offset));
                        if next_pos.within_map_bounds(map_size) {
                            if let Some(next_tile) = tile_storage.get(&next_pos) {
                                bombs.get(next_tile).is_ok()
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }).filter(|v| *v).count();
        
                    let mut current_tile = tiles.get_mut(reveal_tile.entity)?;
                    current_tile.1.0 = match bomb_count {
                        bomb_count if bomb_count > 0 && bomb_count < 10 => bomb_count as u32 - 1,
                        0 => 12,
                        _ => 11
                    };

                    if bomb_count == 0 {
                    for offset in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y].into_iter() {
                        let upos = UVec2::from(tile_pos);
                        let next_pos = TilePos::from(upos.wrapping_add_signed(offset));
                        if next_pos.within_map_bounds(map_size) {
                        if let Some(next_tile) = tile_storage.get(&next_pos) {
                            if let Ok(_) = concealed_tiles.get(next_tile) {
                            commands.trigger(RevealTile {
                                entity: next_tile,
                                is_primary_click: false,
                                tile_pos: next_pos
                            })
                        }
                        };
                    }
                    }}
                }
                Ok(())
            },
        )
        .add_observer(|
            _: On<RevealBombs>,
            mut bombs: Query<&mut TileTextureIndex, (With<Bomb>, With<Concealed>)>,
        | {
            for mut bomb in &mut bombs {
                if bomb.0 == 14 {
                    bomb.0 = 11;
                } else {
                    bomb.0 = 15;
                }
            }

        })
        .run();
}

fn transition_to_start_menu(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Menu);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: (20. + 4.) * 64.,
                min_height: (20. + 4.) * 64.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
