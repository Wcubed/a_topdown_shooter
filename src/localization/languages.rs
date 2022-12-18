use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use fluent::{bundle::FluentBundle, FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::sync::Arc;
use thiserror::Error;
use tracing::error;
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

/// Message that the language loader expects to be available in `.ftl` files,
/// so that the language can be identified with a human-readable name.
const LANGUAGE_NAME_ID: &str = "language_name";

/// `*.ftl` localization file representing a single language for use with [fluent](https://github.com/projectfluent/fluent-rs).
/// The name of the file should be a valid language identifier, for example `en-US.ftl`.
#[derive(TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74bab52"]
pub struct FluentLanguage {
    language_identifier: LanguageIdentifier,
    name: String,
    /// The [IntlLangMemoizer] type needs to be specified because we need the concurrent version
    /// of it. The default [FluentBundle] uses the non-Sync version.
    bundle: FluentBundle<Arc<FluentResource>, IntlLangMemoizer>,
}

impl FluentLanguage {
    pub fn new(
        language_identifier: LanguageIdentifier,
        bundle: FluentBundle<Arc<FluentResource>, IntlLangMemoizer>,
    ) -> FluentLanguage {
        let mut language = FluentLanguage {
            language_identifier,
            name: LANGUAGE_NAME_ID.to_string(),
            bundle,
        };

        language.name = language
            .localize(LANGUAGE_NAME_ID)
            .unwrap_or_else(|_| LANGUAGE_NAME_ID.to_string());
        debug!(
            "Loaded language with id `{}` and name `{}`",
            language.language_identifier, language.name
        );

        language
    }

    pub fn identifier(&self) -> &LanguageIdentifier {
        &self.language_identifier
    }

    pub fn localize(&self, message_id: &str) -> Result<String, LocalizationError> {
        self.localize_with_args(message_id, &[])
    }

    pub fn localize_with_args(
        &self,
        message_id: &str,
        arguments: &[(&str, &str)],
    ) -> Result<String, LocalizationError> {
        let mut fluent_args = FluentArgs::new();
        for (key, value) in arguments {
            fluent_args.set(<&str>::clone(key), <&str>::clone(value));
        }

        let message =
            self.bundle
                .get_message(message_id)
                .ok_or(LocalizationError::MessageNotFound {
                    message_id: message_id.to_string(),
                    language_id: self.language_identifier.clone(),
                })?;
        let pattern = message.value().ok_or(LocalizationError::MessageNotFound {
            message_id: message_id.to_string(),
            language_id: self.language_identifier.clone(),
        })?;

        let mut errors = vec![];
        let result = self
            .bundle
            .format_pattern(pattern, Some(&fluent_args), &mut errors);

        if !errors.is_empty() {
            let errors_string = errors
                .iter()
                .enumerate()
                // TODO (Wybe 2022-06-06): Clean up this error reporting?
                .map(|(i, err)| format!("\n{}: {:?}", i, err))
                .collect::<String>();

            warn!(
                "Errors while localizing `{}` for language `{}`, with arguments {:x?}:{}",
                message_id, self.language_identifier, arguments, errors_string
            );
        }
        Ok(result.to_string())
    }
}

#[derive(Default)]
pub struct FluentLanguageLoader;

impl AssetLoader for FluentLanguageLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let language_id = load_context
                .path()
                .file_stem()
                .ok_or(LanguageLoadingError::AssetWithoutFileName)?
                .to_str()
                .ok_or(LanguageLoadingError::AssetWithoutFileName)?;

            let fluent_string = String::from_utf8_lossy(bytes).to_string();

            let language = load_language_from_fluent_string(fluent_string, language_id)?;

            load_context.set_default_asset(LoadedAsset::new(language));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}

fn load_language_from_fluent_string(
    fluent_string: String,
    lang_id_string: &str,
) -> Result<FluentLanguage, LanguageLoadingError> {
    let resource = FluentResource::try_new(fluent_string).map_err(|(_, errors)| {
        let error_listing = errors
            .iter()
            .enumerate()
            .map(|(i, err)| format!("\n{}: {:?}", i, err))
            .collect::<String>();

        LanguageLoadingError::ParsingFluentFileFailed { error_listing }
    })?;

    let id = lang_id_string.parse::<LanguageIdentifier>()?;

    let mut bundle = FluentBundle::new_concurrent(vec![id.clone()]);
    bundle.add_resource(Arc::new(resource)).map_err(|errors| {
        let error_listing = errors
            .iter()
            .enumerate()
            .map(|(i, err)| format!("\n{}: {:?}", i, err))
            .collect::<String>();

        LanguageLoadingError::ParsingFluentFileFailed { error_listing }
    })?;

    Ok(FluentLanguage::new(id, bundle))
}

#[derive(Error, Debug)]
pub enum LocalizationError {
    #[error("Couldn't find message with id `{message_id}` for language `{language_id}`")]
    MessageNotFound {
        message_id: String,
        language_id: LanguageIdentifier,
    },
}

#[derive(Error, Debug)]
enum LanguageLoadingError {
    #[error("Got a language asset without file name. The file name should be a valid language identifier, like `en-US.ftl`")]
    AssetWithoutFileName,

    #[error(transparent)]
    ParsingLanguageIdentifierFailed(#[from] LanguageIdentifierError),

    #[error("Could not parse `.ftl` file:\n{error_listing}")]
    ParsingFluentFileFailed {
        /// The `FluentResource::try_new` and `FluentBundle.add_resource` functions return
        /// a vector of FluentErrors.
        /// Their messages should be collected in-order, and put in this string.
        error_listing: String,
    },
}
