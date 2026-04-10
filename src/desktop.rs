use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<TauriGoogleNativeAuth<R>> {
  Ok(TauriGoogleNativeAuth(app.clone()))
}

/// Access to the google-native-auth APIs.
pub struct TauriGoogleNativeAuth<R: Runtime>(AppHandle<R>);

impl<R: Runtime> TauriGoogleNativeAuth<R> {
  pub fn signin(&self, _payload: SignInRequest) -> crate::Result<SignInResponse> {
    Ok(SignInResponse {
      success: false,
      data: "".to_string(),
      t: "".to_string(),
    })
  }
}
