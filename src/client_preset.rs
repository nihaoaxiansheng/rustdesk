use hbb_common::config::{self, Config};

// Central place for a "helper client" preset.
//
// Fill these values with your self-hosted RustDesk server details, then rebuild.
// Empty strings are ignored.
const APP_NAME_OVERRIDE: &str = "";

const DEFAULT_OPTIONS: &[(&str, &str)] = &[
    ("custom-rendezvous-server", "vynaris.cn"),
    ("relay-server", "vynaris.cn"),
    ("api-server", ""),
    ("key", "9R1welGbm0P3tJOdRZugpbwpEF33iZSWFFKB9LC7lDs="),
    // Keep remote assistance explicit: the other side still has to accept.
    ("approve-mode", "click"),
];

// These values are re-applied on every launch. Leave empty unless you truly
// want to lock a setting for every build.
const ENFORCED_OPTIONS: &[(&str, &str)] = &[];

// Builtin options affect what the UI shows. These are safe examples for a
// simplified support client. Comment out anything you do not want to hide.
const BUILTIN_OPTIONS: &[(&str, &str)] = &[
    ("hide-server-settings", "Y"),
    ("hide-network-settings", "Y"),
    ("hide-proxy-settings", "Y"),
    ("hide-help-cards", "Y"),
];

// Hard settings are stronger switches checked directly by some UI paths.
// Leave empty unless you really want to disable a feature.
const HARD_SETTINGS: &[(&str, &str)] = &[];

pub fn apply() {
    if !APP_NAME_OVERRIDE.is_empty() {
        *config::APP_NAME.write().unwrap() = APP_NAME_OVERRIDE.to_owned();
    }

    for (key, value) in BUILTIN_OPTIONS {
        config::BUILTIN_SETTINGS
            .write()
            .unwrap()
            .insert((*key).to_owned(), (*value).to_owned());
    }

    for (key, value) in HARD_SETTINGS {
        config::HARD_SETTINGS
            .write()
            .unwrap()
            .insert((*key).to_owned(), (*value).to_owned());
    }

    for (key, value) in DEFAULT_OPTIONS {
        if value.is_empty() {
            continue;
        }
        if Config::get_option(key).is_empty() {
            Config::set_option((*key).to_owned(), (*value).to_owned());
        }
    }

    for (key, value) in ENFORCED_OPTIONS {
        if value.is_empty() {
            continue;
        }
        Config::set_option((*key).to_owned(), (*value).to_owned());
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    crate::ui_interface::refresh_options();
}
