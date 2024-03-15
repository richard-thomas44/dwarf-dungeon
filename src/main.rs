use bevy::{prelude::*, utils::HashMap};

struct TilemapSize {width: usize, height: usize}

#[derive(Resource)]
struct MapDimensions {
    map_size: TilemapSize,        // number of tiles width, height
    tile_size: f32,               // size of each (square) tile in pixels
}

#[derive(Component, Debug)]
struct Tilemap {
    tile: Vec<Option<Entity>>,
    entity: HashMap<Entity, usize>,
    width: usize,
    height: usize,
}

#[derive(Component, Debug)]
struct TilePosition {x: usize, y: usize}
/*
#[derive(Component, Debug)]
struct Tile {tile_pos: TilePosition,
             tile_entity: Entity}
*/
enum TileType {
    Blank,
    EarthCentre,
    StoneCentre,
    OreCentre,
}

impl TileType {
    fn value(&self) -> usize {
        match *self {
            TileType::Blank => 12,
            TileType::EarthCentre => 17,
            TileType::StoneCentre => 21,
            TileType::OreCentre => 25,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Speed {walking: f32, sprinting: f32}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .insert_resource(MapDimensions {
            map_size: TilemapSize {width: 40, height: 30},
            tile_size: 16.,
        })
        .add_systems(Startup, (((
            initialize,
            apply_deferred.after(initialize),spawn_tiles).chain(),
            apply_deferred.after(spawn_tiles),add_walls).chain(),
            apply_deferred.after(add_walls),add_ore).chain(),
         
        )          
        .add_systems(Update, (
            bevy::window::close_on_esc,
//            animate_sprite,

        ))
        .add_systems(FixedUpdate,(
            player_input,
            move_player,
        ).chain())
        .insert_resource(Time::<Fixed>::from_hz(15.))
        .add_event::<ControlPlayerEvent>()
        .run();
}
#[derive(Eq, PartialEq, Hash)]
enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

enum PlayerAction {
    Walk {direction : usize},
    Sprint {direction : usize},
    Attack,
}

struct player_movement{
    translation: Vec3,
    first_index: usize,
    last_index: usize,
}

#[derive(Component)]
struct PlayerMovements {movements: Vec<player_movement>}

fn initialize(mut commands: Commands,
              asset_server: Res<AssetServer>,
              mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
              map_dimensions : Res<MapDimensions >) {
    info!("Initializing");
// The tilemap bidirectional map storing a Vector of entity id's and a hashmap to convert id to coordinates
    commands.spawn(Tilemap {tile: Vec::new(),
                                    entity: HashMap::new(),
                                    width: map_dimensions.map_size.width,
                                    height: map_dimensions.map_size.height,
                                });

// The hero spritesheet

    let texture: Handle<Image> = asset_server.load("images/Characters/Knight1_Move.png");
//    let layout = TextureAtlasLayout::from_grid(Vec2::splat(24.), 4, 8, Some(Vec2::splat(28.)), None);
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(52.), 4, 8, None, None);    
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
//    let animation_indices = AnimationIndices { first: 7, last: 10 };

    let mut player_movements = Vec::new();
    player_movements.push(player_movement {translation: Vec3::Y, first_index: 16, last_index: 19});      // North
    player_movements.push(player_movement {translation: (Vec3::Y + Vec3::X)/((2 as f32).sqrt()), first_index: 12, last_index: 15}); // Northeast  
    player_movements.push(player_movement {translation: Vec3::X, first_index: 8, last_index: 11});       // East
    player_movements.push(player_movement {translation: (Vec3::X - Vec3::Y)/((2 as f32).sqrt()), first_index: 4, last_index: 7}); // Southeast    
    player_movements.push(player_movement {translation: Vec3::NEG_Y, first_index: 0, last_index: 3});        // South
    player_movements.push(player_movement {translation: (Vec3::NEG_X + Vec3::NEG_Y)/((2 as f32).sqrt()), first_index: 28, last_index: 31}); // Southwest   
    player_movements.push(player_movement {translation: Vec3::NEG_X, first_index: 24, last_index: 27});      // West
    player_movements.push(player_movement {translation: (Vec3::NEG_X + Vec3::Y)/((2 as f32).sqrt()), first_index: 20, last_index: 23}); // Northwest 
    
    commands.spawn((SpriteSheetBundle {
        texture: texture.clone(),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        atlas: TextureAtlas {layout: texture_atlas_layout, index: 1},
        ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerMovements{movements: player_movements},
        Speed{walking: 8.0, sprinting: 16.},
        Player,
    ));
}

fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>,
               mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
               map_dimensions : Res<MapDimensions>,
               mut grid_q: Query<&mut Tilemap>,
            ) {
    info!("Spawning tiles");
    let texture = asset_server.load("Cave_Tiles.png");
    let tile_size: Vec2 = Vec2::splat(map_dimensions.tile_size);
    let (columns,rows) = (16, 2);
    let layout = TextureAtlasLayout::from_grid(tile_size, columns, rows, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2dBundle::default());

    let mut grid = grid_q.get_single_mut().unwrap();
    info!("Grid is {:#?}", grid);

    let tile_scale = 1.5;

    for j in 0..map_dimensions.map_size.height {
        for i in 0..map_dimensions.map_size.width {
            let tile = commands.spawn((
                SpriteSheetBundle {
                texture: texture.clone(),
                atlas: TextureAtlas { layout: texture_atlas_layout.clone(), index: TileType::Blank.value() },
                transform: Transform::from_scale(Vec3::splat(tile_scale))
                 .with_translation(Vec3::new( (i as f32 - map_dimensions.map_size.width as f32/2.) *map_dimensions.tile_size*tile_scale,
                                              (j as f32 - map_dimensions.map_size.height as f32/2.)*map_dimensions.tile_size*tile_scale, 1.)),
                ..default()
                }  ,
            )).id();
            grid.tile.push(Some(tile));
            let idx = grid.tile.len() -1;
            assert_eq!(idx, j*map_dimensions.map_size.width + i);
            grid.entity.insert(tile, idx);
        }
    }

}

fn add_walls(grid_q: Query<&mut Tilemap>,
             mut tiles_q: Query<&mut TextureAtlas>) {
     info!("Adding walls");
    let grid = grid_q.get_single().unwrap();

 // Top and bottom edges
    for i in 0..grid.width {
        if let Ok(mut tile) = tiles_q.get_mut(grid.tile[i].unwrap()) { 
            tile.index = TileType::StoneCentre.value();
        }
        if let Ok(mut tile) = tiles_q.get_mut(grid.tile[(grid.height-1)*grid.width + i].unwrap()) {
            tile.index = TileType::StoneCentre.value();
        }
    }
// Left and right edges
    for j in 1..grid.height-1 {
        if let Ok(mut tile) = tiles_q.get_mut(grid.tile[grid.width*j].unwrap()) {
            tile.index = TileType::StoneCentre.value();
        }
        if let Ok(mut tile) = tiles_q.get_mut(grid.tile[grid.width*j + grid.width-1].unwrap()) {
            tile.index = TileType::StoneCentre.value();
        }
    }

}

fn add_ore(
            mut q: Query<&mut TextureAtlas>,
            mut grid_q: Query<&mut Tilemap>,
        ) {

    info!("Adding ore");
    let grid = grid_q.get_single().unwrap();
    let chosen_pos = TilePosition {x: fastrand::usize(1..grid.width-2), y: fastrand::usize(1..grid.height-2)};

    let e = grid.tile[chosen_pos.y*grid.width + chosen_pos.x].unwrap();
    if let Ok(mut tile) = q.get_mut(e)
    {   
        tile.index = TileType::OreCentre.value();
    };
}

// Abstractions for player movement

#[derive(Event)]
struct ControlPlayerEvent(PlayerAction);

// Collect play input by polling keypresses. Check first for WASD key combinations, then single WASD directions

fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_control_player: EventWriter<ControlPlayerEvent>,
    ) { 
    let direction = 'block: {
        if keyboard_input.pressed(KeyCode::KeyW) {
            if keyboard_input.pressed(KeyCode::KeyA) {
                break 'block Some(7); // Northwest
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                break 'block Some(1); // Northeast
            }
                break 'block Some(0); // North
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            if keyboard_input.pressed(KeyCode::KeyA) {
                break 'block Some(5); // Southwest
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                break 'block Some(3); // Southeast
            }
            break 'block Some(4); // South
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            break 'block Some(6); // West
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            break 'block Some(2); // East
        }
        None
    };
    if let Some(dir) = direction {
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            ev_control_player.send(ControlPlayerEvent(PlayerAction::Sprint {direction : dir}));
        } else {
            ev_control_player.send(ControlPlayerEvent(PlayerAction::Walk {direction : dir}));
            }  
    }
}   

fn move_player(
    mut ev_control_player: EventReader<ControlPlayerEvent>,
    mut q: Query<(&mut Transform, &Speed, &PlayerMovements), With<Player>>,
    mut q_atlas: Query <&mut TextureAtlas, With<Player>>,
    ) {
    let (mut t, speed, m) = q.get_single_mut().unwrap();

    for ev in ev_control_player.read() {
        match &ev.0 {
            PlayerAction::Walk {direction} => {
                t.translation += m.movements[*direction].translation * speed.walking;
                let mut atlas = q_atlas.single_mut();
                atlas.index = if atlas.index >= m.movements[*direction].last_index || atlas.index < m.movements[*direction].first_index {
                    m.movements[*direction].first_index
                } else {
                    atlas.index + 1
                };
            }                    
            PlayerAction::Sprint {direction} => {
                t.translation += m.movements[*direction].translation * speed.sprinting;
                let mut atlas = q_atlas.single_mut();
                atlas.index = if atlas.index >= m.movements[*direction].last_index || atlas.index < m.movements[*direction].first_index {
                    m.movements[*direction].first_index
                } else {
                    atlas.index + 1
                };
            }          
            PlayerAction::Attack => todo!()
        }
    }
}
