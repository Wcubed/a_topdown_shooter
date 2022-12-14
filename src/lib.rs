use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use tracing::info;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Main)
                    .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                        "dynamic_assets.assets",
                    ])
                    .with_collection::<ImageAssets>(),
            )
            .add_state(GameState::AssetLoading)
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(spawn_player))
            .add_system(hello_system);
    }
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(key = "image.player")]
    player: Handle<Image>,
}

#[derive(Component)]
struct Player;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Main,
}

fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn(Camera2dBundle::default());

    let mut input_map = InputMap::new([(KeyCode::Space, Action::HelloAction)]);

    input_map.insert_chord(
        [
            InputKind::Modifier(Modifier::Shift),
            InputKind::Keyboard(KeyCode::H),
        ],
        Action::HelloAction,
    );

    commands
        .spawn(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map,
        })
        .insert(SpriteBundle {
            texture: image_assets.player.clone(),
            ..default()
        })
        .insert(Player);
}

fn hello_system(query: Query<&ActionState<Action>, With<Player>>) {
    if let Ok(action_state) = query.get_single() {
        if action_state.just_pressed(Action::HelloAction) {
            info!("Hello World!");
        }
    }
}

#[derive(Actionlike, Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum Action {
    HelloAction,
}
