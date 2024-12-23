use bevy::{input::InputSystem, prelude::*};
use crate::build_map::Collider;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Speed {walking: f32, sprinting: f32}

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, (
            initialize_character,
        ))
        .add_systems(Update, (
            move_player.after(InputSystem),
            animate_player,
        ))
        .insert_resource(Time::<Fixed>::from_hz(15.))
        .add_event::<ControlPlayerEvent>();
}

pub enum PlayerAction {
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
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(52), 4, 8, None, None);    // Migration to 0.14 changed Vec2 to UVec2
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
    
    commands.spawn((SpriteBundle {
        texture: texture.clone(),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
        },
        TextureAtlas {layout: texture_atlas_layout, index: 1,
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
pub struct ControlPlayerEvent(pub PlayerAction);


// Move player in correct direction if a movement key is being pressed

fn move_player(
    time: Res<Time>,
    mut ev_control_player: EventReader<ControlPlayerEvent>,
    mut q: Query<(&mut PlayerStatus, &mut FacingDirection, &mut Transform, &Speed, &PlayerMovements), With<Player>>,
    colliders: Query<(Entity, &Transform), (With<Collider>, Without<Player>)>,
) {
    let (mut status, mut facing, mut t, speed, m) = q.get_single_mut().unwrap();

    for ev in ev_control_player.read() {
        let mut proposed_trans = t.translation;
        match &ev.0 {
            PlayerAction::Walk {direction} => {
                proposed_trans += m.movements[*direction].translation * speed.walking * time.delta_seconds() * 30.;
                facing.direction = *direction;
                status.action = PlayerAction::Walk { direction: *direction };
            }                    
            PlayerAction::Sprint {direction} => {
                proposed_trans += m.movements[*direction].translation * speed.sprinting * time.delta_seconds() * 30.;
                facing.direction = *direction;
                status.action = PlayerAction::Sprint { direction: *direction };
            }          
            PlayerAction::Attack => todo!(),
            PlayerAction::Stand {direction: _}=> {},
        }

// Check for collisions
        let mut collision_type = 0;
        for (_e, t) in colliders.iter() {
//            info!("Checking against {:#?}", t.translation);
            if t.translation.distance(proposed_trans) < 26.{            // Simply check distance. Not as good as checking overlapping bounding boxes
//                info!("Collided with {:#?} at {:#?}", e, t);
                collision_type = 1;
            }
        }
        if collision_type == 0 {
            t.translation = proposed_trans;
        }
    }

// TODO: start the animation immediately, do not just wait for the next animation timer tick

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
