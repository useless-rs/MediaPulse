use std::ffi::{OsStr, OsString};

use serde::{Deserialize, Serialize};

use super::options::{OptionArity, UiOption, apply_ui_option, classify};

/// UI-owned state extracted from mpv-compatible arguments.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct UiOverrides {
    pub volume: Option<f64>,
    pub paused: Option<bool>,
    pub fullscreen: Option<bool>,
    pub speed: Option<f64>,
}

/// Original arguments plus a side-effect-free UI projection.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedInvocation {
    engine_args: Vec<OsString>,
    ui_overrides: UiOverrides,
    lists_options: bool,
}

impl ParsedInvocation {
    /// Arguments that must be passed to mpv without reconstruction.
    #[must_use]
    pub fn engine_args(&self) -> &[OsString] {
        &self.engine_args
    }

    /// State mirrored into the application shell.
    #[must_use]
    pub const fn ui_overrides(&self) -> &UiOverrides {
        &self.ui_overrides
    }

    /// Whether mpv must handle its own option listing.
    #[must_use]
    pub const fn lists_options(&self) -> bool {
        self.lists_options
    }
}

/// Parse the local UI registry while preserving unknown arguments byte-for-byte.
pub fn parse_invocation<I>(args: I) -> ParsedInvocation
where
    I: IntoIterator<Item = OsString>,
{
    let engine_args: Vec<OsString> = args.into_iter().collect();
    let mut ui_overrides = UiOverrides::default();
    let mut lists_options = false;
    let mut index = 0;

    while let Some(raw) = engine_args.get(index) {
        let Some(text) = raw.to_str() else {
            index += 1;
            continue;
        };

        if text == "--" {
            break;
        }
        if text == "--list-options" {
            lists_options = true;
            index += 1;
            continue;
        }

        let Some(token) = ParsedToken::parse(text) else {
            index += 1;
            continue;
        };
        let Some(arity) = classify(token.name) else {
            index += 1;
            continue;
        };

        let separate = engine_args
            .get(index + 1)
            .and_then(|value| OsStr::to_str(value))
            .filter(|value| is_value_candidate(arity, value));
        let value = token.value.or(separate);
        let consumed = if token.value.is_none() && separate.is_some() {
            2
        } else {
            1
        };

        if arity == OptionArity::Value && value.is_none() {
            index += 1;
            continue;
        }

        apply_ui_option(
            &mut ui_overrides,
            UiOption {
                name: token.name,
                value,
                negated: token.negated,
            },
        );
        index += consumed;
    }

    ParsedInvocation {
        engine_args,
        ui_overrides,
        lists_options,
    }
}

struct ParsedToken<'a> {
    name: &'a str,
    value: Option<&'a str>,
    negated: bool,
}

impl<'a> ParsedToken<'a> {
    fn parse(text: &'a str) -> Option<Self> {
        let body = text.strip_prefix("--").or_else(|| text.strip_prefix('-'))?;
        if body.is_empty() {
            return None;
        }
        if let Some((name, value)) = body.split_once('=') {
            return Some(Self {
                name,
                value: Some(value),
                negated: false,
            });
        }
        if let Some(name) = body.strip_prefix("no-") {
            return Some(Self {
                name,
                value: None,
                negated: true,
            });
        }
        Some(Self {
            name: body,
            value: None,
            negated: false,
        })
    }
}

fn is_value_candidate(arity: OptionArity, value: &str) -> bool {
    match arity {
        OptionArity::Value => {
            !value.starts_with('-') || value.parse::<f64>().is_ok_and(f64::is_finite)
        }
        OptionArity::Flag => super::options::parse_bool(value).is_some(),
    }
}
