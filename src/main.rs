use bevy::prelude::*;
use build_map::build_map_plugin;

mod build_map;
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Speed {walking: f32, sprinting: f32}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins((
            build_map_plugin,
        ))
        .add_systems(Startup, (
            initialize_character,
        ))          
        .add_systems(Update, (
            bevy::window::close_on_esc,
            animate_player,
            (player_input2,
            move_player).chain(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(15.))
        .add_event::<ControlPlayerEvent>()
        .run();
}
enum PlayerAction {
    Stand {direction : usize},
    Walk {direction : usize},
    Sprint {direction : usize},
    Attack,
}
#[derive(Component)]
struct PlayerStatus {action: PlayerAction}

struct PlayerMovement{
    translation: Vec3,
    first_index: usize,
    last_index: usize,
}

#[derive(Component)]
struct PlayerMovements {movements: Vec<PlayerMovement>}

#[derive(Component)]
struct FacingDirection{direction: usize}



fn initialize_character(mut commands: Commands,
              asset_server: Res<AssetServer>,
              mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
            ) {
    info!("Initializing character");

// The hero spritesheet

    let texture: Handle<Image> = asset_server.load("images/Characters/Knight1_Move.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(52.), 4, 8, None, None);    
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut player_movements = Vec::new();
    player_movements.push(PlayerMovement {translation: Vec3::Y, first_index: 16, last_index: 19});      // North
    player_movements.push(PlayerMovement {translation: (Vec3::Y + Vec3::X)/((2 as f32).sqrt()), first_index: 12, last_index: 15}); // Northeast  
    player_movements.push(PlayerMovement {translation: Vec3::X, first_index: 8, last_index: 11});       // East
    player_movements.push(PlayerMovement {translation: (Vec3::X - Vec3::Y)/((2 as f32).sqrt()), first_index: 4, last_index: 7}); // Southeast    
    player_movements.push(PlayerMovement {translation: Vec3::NEG_Y, first_index: 0, last_index: 3});        // South
    player_movements.push(PlayerMovement {translation: (Vec3::NEG_X + Vec3::NEG_Y)/((2 as f32).sqrt()), first_index: 28, last_index: 31}); // Southwest   
    player_movements.push(PlayerMovement {translation: Vec3::NEG_X, first_index: 24, last_index: 27});      // West
    player_movements.push(PlayerMovement {translation: (Vec3::NEG_X + Vec3::Y)/((2 as f32).sqrt()), first_index: 20, last_index: 23}); // Northwest 
    
    commands.spawn((SpriteSheetBundle {
        texture: texture.clone(),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        atlas: TextureAtlas {layout: texture_atlas_layout, index: 1},
        ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerMovements{movements: player_movements},
        Speed{walking: 8.0, sprinting: 16.},
        FacingDirection{direction: 0},
        PlayerStatus{action : PlayerAction::Stand {direction:0} },
        Player,
    ));
}

// Abstractions for player movement

#[derive(Event)]
struct ControlPlayerEvent(PlayerAction);

fn player_input2(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_control_player: EventWriter<ControlPlayerEvent>,
    ) {
    let keys : u8 = keyboard_input.pressed(KeyCode::KeyW) as u8
                 | (keyboard_input.pressed(KeyCode::KeyA) as u8) << 1
                 | (keyboard_input.pressed(KeyCode::KeyS) as u8) << 2
                 | (keyboard_input.pressed(KeyCode::KeyD) as u8) << 3;

    let dir = match keys {
        0b0001 => 0,
        0b1001 => 1,
        0b1000 => 2,
        0b1100 => 3,
        0b0100 => 4,
        0b0110 => 5,
        0b0010 => 6,
        0b0011 => 7,
        _      => {return},
    };
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        ev_control_player.send(ControlPlayerEvent(PlayerAction::Sprint {direction : dir}));
    } else {
        ev_control_player.send(ControlPlayerEvent(PlayerAction::Walk {direction : dir}));
        }  
}

// Collect player input by polling keypresses. Check first for WASD key combinations, then single WASD directions

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

// Move player in correct direction if a movement key is being pressed

fn move_player(
    mut ev_control_player: EventReader<ControlPlayerEvent>,
    mut q: Query<(&mut PlayerStatus, &mut FacingDirection, &mut Transform, &Speed, &PlayerMovements), With<Player>>,
) {
    let (mut status, mut facing, mut t, speed, m) = q.get_single_mut().unwrap();

    for ev in ev_control_player.read() {
        match &ev.0 {
            PlayerAction::Walk {direction} => {
                t.translation += m.movements[*direction].translation * speed.walking;
                facing.direction = *direction;
                status.action = PlayerAction::Walk { direction: *direction };
            }                    
            PlayerAction::Sprint {direction} => {
                t.translation += m.movements[*direction].translation * speed.sprinting;
                facing.direction = *direction;
                status.action = PlayerAction::Sprint { direction: *direction };
            }          
            PlayerAction::Attack => todo!(),
            PlayerAction::Stand {direction: _}=> {},
        }
    }
}

// Iterate through player sprite sheet if player has pressed or held a key this tick

fn animate_player(
    mut ev_control_player: EventReader<ControlPlayerEvent>,
    time: Res<Time>,
    mut q: Query<(&PlayerMovements, &mut TextureAtlas, &mut AnimationTimer), With<Player>>,
) {
    let (m, mut atlas, mut timer) = q.get_single_mut().unwrap();
    for ev in ev_control_player.read() {
        match ev.0 {
            PlayerAction::Walk {direction} | PlayerAction::Sprint {direction } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    atlas.index = if atlas.index >= m.movements[direction].last_index || atlas.index < m.movements[direction].first_index {
                        m.movements[direction].first_index
                    } else {
                        atlas.index + (timer.times_finished_this_tick() as usize % (m.movements[direction].last_index-m.movements[direction].first_index))
                    };
                }
            },
            _ => (),
        }
    }
}