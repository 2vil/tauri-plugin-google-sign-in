use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the google-native-auth APIs.
pub trait TauriGoogleNativeAuthExt<R: Runtime> {
  fn google_native_auth(&self) -> &TauriGoogleNativeAuth<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TauriGoogleNativeAuthExt<R> for T {
  fn google_native_auth(&self) -> &TauriGoogleNativeAuth<R> {
    self.state::<TauriGoogleNativeAuth<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("google-native-auth")
  .invoke_handler(tauri::generate_handler![
    commands::signin
  ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let google_native_auth = mobile::init(app, api)?;
      #[cfg(desktop)]
      let google_native_auth = desktop::init(app, api)?;
      app.manage(google_native_auth);
      Ok(())
    })
    .build()
}
