use std::string::FromUtf8Error;

use crate::bindings::PluginError;
use scraper::error::SelectorErrorKind;

impl From<SelectorErrorKind<'_>> for PluginError {
    fn from(value: SelectorErrorKind<'_>) -> Self {
        PluginError {
            err: value.to_string(),
        }
    }
}

impl From<anyhow::Error> for PluginError {
    fn from(value: anyhow::Error) -> Self {
        PluginError {
            err: value.to_string(),
        }
    }
}

impl From<FromUtf8Error> for PluginError {
    fn from(value: FromUtf8Error) -> Self {
        PluginError {
            err: value.to_string(),
        }
    }
}
