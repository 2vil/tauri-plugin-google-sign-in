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

#[cfg(desktop)]
use crate::desktop::TauriGoogleNativeAuth;
#[cfg(mobile)]
use crate::mobile::TauriGoogleNativeAuth;

#[cfg(not(any(desktop, mobile)))]
pub struct TauriGoogleNativeAuth<R: Runtime>(tauri::AppHandle<R>);

#[cfg(not(any(desktop, mobile)))]
impl<R: Runtime> TauriGoogleNativeAuth<R> {
  pub fn signin(&self, _payload: SignInRequest) -> crate::Result<SignInResponse> {
    Err(crate::Error::from("Google Native Auth is not supported on this platform"))
  }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the google-native-auth APIs.
pub trait TauriGoogleNativeAuthExt<R: Runtime> {
  fn google_native_auth(&self) -> &TauriGoogleNativeAuth<R>;
}

impl<R: Runtime, T: Manager<R>> TauriGoogleNativeAuthExt<R> for T {
  fn google_native_auth(&self) -> &TauriGoogleNativeAuth<R> {
    self.state::<TauriGoogleNativeAuth<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  #[allow(unused_mut)]
  let mut builder = Builder::new("google-native-auth");

  #[cfg(any(desktop, mobile))]
  {
    builder = builder.invoke_handler(tauri::generate_handler![commands::signin]);
  }

  builder
    .setup(|app, _api| {
      #[cfg(mobile)]
      {
        let google_native_auth = mobile::init(app, _api)?;
        app.manage(google_native_auth);
      }
      #[cfg(desktop)]
      {
        let google_native_auth = desktop::init(app, _api)?;
        app.manage(google_native_auth);
      }
      Ok(())
    })
    .build()
}
