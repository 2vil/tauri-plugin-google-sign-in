use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::TauriGoogleNativeAuthExt;

#[command]
pub(crate) async fn signin<R: Runtime>(
    app: AppHandle<R>,
    payload: SignInRequest,
) -> Result<SignInResponse> {
    app.google_native_auth().signin(payload)
}
