use bevy::pbr::RenderLightmaps;
use bevy::{input::InputSystem, prelude::*};
use crate::build_map::Collider;
use crate::game_state::GameStateSet;
use crate::physics::MotionState;
use crate::input_control::Directions;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Speed {walking: f32, sprinting: f32}

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, (
            initialize_character,
        ).in_set(GameStateSet::InGameSet))
        .add_systems(Update, (
            move_player2.after(InputSystem),
            animate_player,
        ).in_set(GameStateSet::InGameSet))  
        .insert_resource(Time::<Fixed>::from_hz(15.))
        .add_event::<ControlPlayerEvent>();
}

pub enum PlayerAction {
    Stand {direction : Directions},
    Walk {direction : Directions},
    Sprint {direction : Directions},
    Jump {direction: Directions},
    Attack,
}
#[derive(Component)]
struct PlayerStatus {action: PlayerAction}

struct PlayerMovement{
    direction: Directions,
    translation: Vec3,
    first_index: usize,
    last_index: usize,
}

#[derive(Component)]
struct PlayerMovements {movements: Vec<PlayerMovement>}

#[derive(Component)]
struct FacingDirection{direction: Directions}



fn initialize_character(mut commands: Commands,
              asset_server: Res<AssetServer>,
              mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
            ) {
    info!("Initializing character");

// The hero spritesheet

    let texture: Handle<Image> = asset_server.load("images/Characters/Knight1_Move.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(52), 4, 8, None, None);    // Migration to 0.14 changed Vec2 to UVec2
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut player_movements = Vec::new();
    player_movements.push(PlayerMovement {direction: Directions::Up, translation: Vec3::Y, first_index: 16, last_index: 19});      
    player_movements.push(PlayerMovement {direction: Directions::UpRight, translation: (Vec3::Y + Vec3::X)/((2 as f32).sqrt()), first_index: 12, last_index: 15}); 
    player_movements.push(PlayerMovement {direction: Directions::Right, translation: Vec3::X, first_index: 8, last_index: 11});  
    player_movements.push(PlayerMovement {direction: Directions::DownRight, translation: (Vec3::X - Vec3::Y)/((2 as f32).sqrt()), first_index: 4, last_index: 7}); 
    player_movements.push(PlayerMovement {direction: Directions::Down, translation: Vec3::NEG_Y, first_index: 0, last_index: 3});
    player_movements.push(PlayerMovement {direction: Directions::DownLeft, translation: (Vec3::NEG_X + Vec3::NEG_Y)/((2 as f32).sqrt()), first_index: 28, last_index: 31});  
    player_movements.push(PlayerMovement {direction: Directions::Left, translation: Vec3::NEG_X, first_index: 24, last_index: 27});      // West
    player_movements.push(PlayerMovement {direction: Directions::UpLeft ,translation: (Vec3::NEG_X + Vec3::Y)/((2 as f32).sqrt()), first_index: 20, last_index: 23});
    
    commands.spawn((Sprite {
        image: texture.clone(),
        texture_atlas: Some(TextureAtlas {layout: texture_atlas_layout, index: 1,
            ..default()
            }),
        ..default()
        },
        Transform::from_scale(Vec3::splat(3.0)),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerMovements{movements: player_movements},
        MotionState{position: Vec3::new(-0. ,55., 1.), velocity: Vec3::Y*(0.), rotation: 0., grounded: false},
        Speed{walking: 8.0, sprinting: 16.},
        FacingDirection{direction: Directions::Right},
        PlayerStatus{action : PlayerAction::Stand {direction:Directions::Right} },

        Player,
    ));
}

// Abstractions for player movement

#[derive(Event)]
pub struct ControlPlayerEvent(pub PlayerAction);

// Adjust player velocity in response to control-input events. This doesn't actually move the player; that happens in physics()
 
fn move_player2(
    time: Res<Time>,
    mut ev_control_player: EventReader<ControlPlayerEvent>,
    mut q: Query<(&mut PlayerStatus, &mut FacingDirection, &mut MotionState, &Speed, &PlayerMovements), With<Player>>,
) {
    let (mut status, mut facing, mut motion, speed, m) = q.get_single_mut().unwrap();
    let delta = motion.velocity * Vec3::X * 0.1;
    if motion.grounded {
        motion.velocity -= delta;                                     // damp horizontal motion so player slows down unless keys held
    }
    for ev in ev_control_player.read() {
        match &ev.0 {
            PlayerAction::Walk {direction} => {
                if motion.grounded && (*direction == Directions::Left || *direction == Directions::Right) {
                    motion.velocity += m.movements[*direction as usize].translation * speed.walking * time.delta_secs() * 300.;
                    facing.direction = *direction;
                    status.action = PlayerAction::Walk { direction: *direction };
                }
            }
            PlayerAction::Sprint {direction} => {
                if motion.grounded && (*direction == Directions::Left || *direction == Directions::Right) {
                    motion.velocity += m.movements[*direction as usize].translation * speed.walking * time.delta_secs() * 600.;
                    facing.direction = *direction;
                    status.action = PlayerAction::Sprint { direction: *direction };
                }
            }
            PlayerAction::Stand {direction: _}=> todo!(),
            PlayerAction::Attack => todo!(),
            PlayerAction::Jump {direction} => {
                if motion.grounded {
                    motion.velocity += Vec3::Y * 200.;
                    motion.grounded = false;
                }
            }
        }
    }
}

// Iterate through player sprite sheet if player has pressed or held a key this tick

fn animate_player(
    mut ev_control_player: EventReader<ControlPlayerEvent>,
    time: Res<Time>,
    mut q: Query<(&PlayerMovements, &mut Sprite, &mut AnimationTimer, &MotionState), With<Player>>,
) {
    let (m, mut sprite, mut timer, motion) = q.get_single_mut().unwrap();
    for ev in ev_control_player.read() {
        match ev.0 {
            PlayerAction::Walk {direction} | PlayerAction::Sprint {direction } => {
                timer.tick(time.delta());
                if timer.just_finished() && motion.grounded {
                    sprite.texture_atlas.as_mut().unwrap().index = if sprite.texture_atlas.as_mut().unwrap().index >= m.movements[direction as usize].last_index || sprite.texture_atlas.as_mut().unwrap().index < m.movements[direction as usize].first_index {
                        m.movements[direction as usize].first_index
                    } else {
                        sprite.texture_atlas.as_mut().unwrap().index + (timer.times_finished_this_tick() as usize % (m.movements[direction as usize].last_index-m.movements[direction as usize].first_index))
                    };
                }
            },
            _ => (),
        }
    }
}
