use super::invocation::UiOverrides;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum OptionArity {
    Flag,
    Value,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct UiOption<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub negated: bool,
}

pub(super) const fn classify(name: &str) -> Option<OptionArity> {
    match name.as_bytes() {
        b"volume" | b"speed" => Some(OptionArity::Value),
        b"pause" | b"fullscreen" => Some(OptionArity::Flag),
        _ => None,
    }
}

pub(super) fn apply_ui_option(overrides: &mut UiOverrides, option: UiOption<'_>) {
    match option.name {
        "volume" => set_volume(overrides, option.value),
        "speed" => set_speed(overrides, option.value),
        "pause" => set_flag(&mut overrides.paused, option),
        "fullscreen" => set_flag(&mut overrides.fullscreen, option),
        _ => {}
    }
}

fn set_volume(overrides: &mut UiOverrides, value: Option<&str>) {
    let Some(value) = value.and_then(|value| value.parse::<f64>().ok()) else {
        return;
    };
    if value.is_finite() {
        overrides.volume = Some(value);
    }
}

fn set_speed(overrides: &mut UiOverrides, value: Option<&str>) {
    let Some(value) = value.and_then(|value| value.parse::<f64>().ok()) else {
        return;
    };
    if value.is_finite() && value > 0.0 {
        overrides.speed = Some(value);
    }
}

fn set_flag(target: &mut Option<bool>, option: UiOption<'_>) {
    if let Some(value) = flag_value(option) {
        *target = Some(value);
    }
}

fn flag_value(option: UiOption<'_>) -> Option<bool> {
    match option.value {
        Some(value) => parse_bool(value),
        None => Some(!option.negated),
    }
}

pub(super) fn parse_bool(value: &str) -> Option<bool> {
    match value.to_ascii_lowercase().as_str() {
        "1" | "yes" | "true" | "on" => Some(true),
        "0" | "no" | "false" | "off" => Some(false),
        _ => None,
    }
}
