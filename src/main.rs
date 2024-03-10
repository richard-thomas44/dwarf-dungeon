use bevy::{prelude::*, render::texture, utils::HashMap};

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
        ))
        .run();
}
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
                       
    let texture: Handle<Image> = asset_server.load("PixelCrawler1.8/Heroes/Knight/Run/Run-Sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(64.), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn(SpriteSheetBundle {
        texture: texture.clone(),
        transform: Transform::from_scale(Vec3::splat(1.0)),
        atlas: TextureAtlas {layout: texture_atlas_layout, index: 1},
        ..default()
        }
    );
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
//                TilePosition{x: i, y: j},
            )).id();
            grid.tile.push(Some(tile));
            let idx = grid.tile.len() -1;
            assert_eq!(idx, j*map_dimensions.map_size.width + i);
            grid.entity.insert(tile, idx);
        }
    }

}

fn add_walls(mut commands: Commands,mut grid_q: Query<&mut Tilemap>,
             mut tiles_q: Query<&mut TextureAtlas>) {
     info!("Adding walls");
    let mut grid = grid_q.get_single_mut().unwrap();

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

fn add_ore(mut commands: Commands,
//           mut q: Query<(Entity, &mut TextureAtlas, &mut TilePosition)>,
            mut q: Query<&mut TextureAtlas>,
            mut grid_q: Query<&mut Tilemap>,
        ) {

    info!("Adding ore");
    let mut grid = grid_q.get_single_mut().unwrap();
    let chosen_pos = TilePosition {x: fastrand::usize(1..grid.width-2), y: fastrand::usize(1..grid.height-2)};

    let e = grid.tile[chosen_pos.y*grid.width + chosen_pos.x].unwrap();
    if let Ok(mut tile) = q.get_mut(e)
    {   
        tile.index = TileType::OreCentre.value();
    };
}
