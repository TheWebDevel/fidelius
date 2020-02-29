extern crate clap;
extern crate clipboard;
extern crate libreauth;
extern crate preferences;

use clap::{load_yaml, App};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use libreauth::oath::TOTPBuilder;
use preferences::{AppInfo, Preferences, PreferencesMap};

const APP_INFO: AppInfo = AppInfo {
    name: "fidelius",
    author: "Secret keeper",
};

const KEYS_PATH: &str = "fidelius/keys";

// Generate TOTP
fn generate_totp(service: &str) {
    // Retrieve the user's preferences
    let load_keys = PreferencesMap::<String>::load(&APP_INFO, KEYS_PATH);
    let mut service_key = service.to_string();
    service_key.push_str("_key");
    match load_keys {
        Ok(keys) => {
            let key = keys[&service_key].to_string();
            let code = TOTPBuilder::new()
                .base32_key(&key)
                .finalize()
                .unwrap()
                .generate();
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(code.to_owned()).unwrap();
            println!("✅ OTP Copied to Clipboard!")
        }
        Err(_e) => println!("❌ Key not set. Set using --set-key."),
    }
}

// Generate Key for a service
fn set_key(service: &str, key: &str) {
    let mut settings: PreferencesMap<String> = PreferencesMap::new();
    let mut service_key = service.to_string();
    service_key.push_str("_key");
    settings.insert(service_key.into(), key.into());
    let save_result = settings.save(&APP_INFO, KEYS_PATH);
    if save_result.is_ok() {
        generate_totp(service)
    }
    assert!(save_result.is_ok());
}

// Main Method
fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("SERVICE") {
        if let Some(service) = matches.value_of("SERVICE") {
            if matches.is_present("generate") {
                generate_totp(service);
            }

            if let Some(key) = matches.value_of("setup") {
                set_key(service, key);
            }
        }
    }
}
