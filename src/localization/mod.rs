mod languages;

use crate::localization::languages::{FluentLanguage, FluentLanguageLoader, LocalizationError};
use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use itertools::Itertools;

const DEFAULT_LANGUAGE_IDENTIFIER: &str = "en-US";

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<FluentLanguage>()
            .init_asset_loader::<FluentLanguageLoader>()
            .add_system_set(
                SystemSet::on_exit(GameState::AssetLoading).with_system(initialize_localization),
            );
    }
}

/// This resource only exists during the [GameState::AssetLoading] state, and is
/// used there so that the asset loader plugin can load the language files.
///
/// Afterwards, the language structs will be moved to the [Localization] resource by
/// [initialize_localization]
#[derive(AssetCollection, Resource)]
pub struct LocalizationAssets {
    #[asset(key = "language_files", collection(typed))]
    languages: Vec<Handle<FluentLanguage>>,
}

#[derive(Resource)]
pub struct Localization {
    // TODO (Wybe 2022-12-18): Add a configurable fallback language
    //   (because falling back on english doesn't always make sense. For example: Frysian would probably want Dutch as fallback).
    //   And _only_ after that fallback language fails, will we try english.
    current_language_index: usize,
    languages: Vec<FluentLanguage>,
}

impl Localization {
    pub fn localize(&self, message_id: &str) -> String {
        // TODO (Wybe 2022-12-18): Error handling, or fallback, or maybe if the language is not found we want to auto-select english?
        let language = &self.languages[self.current_language_index];

        match language.localize(message_id) {
            Ok(result) => result,
            Err(error) => {
                warn!("{}", error);
                message_id.to_string()
            }
        }
    }
}

/// System that should be called after all the languages in the `LocalizationAssets` resource has been loaded.
/// Removes all [FluentLanguage]s listed in [LocalizationAssets] and creates a [Localization] resource for
/// convenience.
fn initialize_localization(
    mut commands: Commands,
    localization_assets: Res<LocalizationAssets>,
    mut language_assets: ResMut<Assets<FluentLanguage>>,
) {
    let languages: Vec<FluentLanguage> = localization_assets
        .languages
        .iter()
        .filter_map(|handle| language_assets.remove(handle))
        .collect();

    let default_language_index = languages
        .iter()
        .find_position(|language| *language.identifier() == DEFAULT_LANGUAGE_IDENTIFIER)
        .map(|(index, _)| index)
        .expect("Cannot start, need at least a language file for English (en-US.ftl).");

    commands.insert_resource(Localization {
        current_language_index: default_language_index,
        languages,
    });

    commands.remove_resource::<LocalizationAssets>();
}
