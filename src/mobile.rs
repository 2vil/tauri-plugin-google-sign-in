use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_google_native_auth);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<TauriGoogleNativeAuth<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.google_native_auth", "ThePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_google_native_auth)?;
  Ok(TauriGoogleNativeAuth(handle))
}

/// Access to the google-native-auth APIs.
pub struct TauriGoogleNativeAuth<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> TauriGoogleNativeAuth<R> {
  #[cfg(target_os = "android")]
  pub fn signin(&self, payload: SignInRequest) -> crate::Result<SignInResponse> {
    self
      .0
      .run_mobile_plugin("signin", payload)
      .map_err(Into::into)
  }
}
