use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use tracing::info;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_startup_system(spawn_player)
            .add_system(hello_system);
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::Space, Action::HelloAction)]),
        })
        .insert(Player);
}

fn hello_system(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();

    if action_state.just_pressed(Action::HelloAction) {
        info!("Hello World!");
    }
}

#[derive(Actionlike, Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum Action {
    HelloAction,
}
