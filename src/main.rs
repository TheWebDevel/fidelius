extern crate clap;
extern crate clipboard;
extern crate libreauth;
extern crate preferences;
#[macro_use]
extern crate human_panic;

use clap::{load_yaml, App};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use libreauth::oath::TOTPBuilder;
use preferences::{AppInfo, Preferences, PreferencesMap};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const APP_INFO: AppInfo = AppInfo {
  name: "fidelius",
  author: "Secret keeper",
};

const KEYS_PATH: &str = "/";

fn remove_whitespace(s: &str) -> String {
  s.chars().filter(|c| !c.is_whitespace()).collect()
}

// Export config file to a target destination
fn export(path: &str) -> std::io::Result<()> {
  let base_path = preferences::prefs_base_dir().unwrap();
  let file_path = PathBuf::from("fidelius/keys.prefs.json");
  let keys_path = base_path.join(file_path.clone());
  fs::copy(keys_path, path)?;
  Ok(())
}

// Import config file to a target destination
fn import(path: &str) -> std::io::Result<()> {
  let base_path = preferences::prefs_base_dir().unwrap();
  if !Path::new(&base_path.join("fidelius")).exists() {
    fs::create_dir(&base_path.join("fidelius"))?;
  }
  let file_path = PathBuf::from("fidelius/keys.prefs.json");
  let keys_path = base_path.join(file_path.clone());
  fs::copy(path, keys_path)?;
  Ok(())
}

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
  let cleaned_key = remove_whitespace(key);
  settings.insert(service_key.into(), cleaned_key.into());
  let save_result = settings.save(&APP_INFO, KEYS_PATH);
  if save_result.is_ok() {
    generate_totp(service)
  }
}

// Main Method
fn main() {
  // Humanized Panic
  setup_panic!(Metadata {
    name: env!("CARGO_PKG_NAME").into(),
    version: env!("CARGO_PKG_VERSION").into(),
    authors: "Sathish Kumar <sathish@skcript.com>".into(),
    homepage: "https://github.com/thewebdevel/fidelius".into(),
  });
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
  } else {
    // You can get the independent subcommand matches (which function exactly like App matches)
    if let Some(ref matches) = matches.subcommand_matches("export") {
      // Safe to use unwrap() because of the required() option
      let path = matches.value_of("path").unwrap();
      let exported = export(path);
      match exported {
        Ok(_ok) => println!("✅ Config file exported to : {}", path),
        Err(err) => println!("❌ Oops. Unable to export file : {}", err),
      }
    }

    // You can get the independent subcommand matches (which function exactly like App matches)
    if let Some(ref matches) = matches.subcommand_matches("import") {
      // Safe to use unwrap() because of the required() option
      let path = matches.value_of("path").unwrap();
      let imported = import(path);
      match imported {
        Ok(_ok) => println!("✅ Config file imported : {}", path),
        Err(err) => println!("❌ Oops. Unable to import file : {}", err),
      }
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
