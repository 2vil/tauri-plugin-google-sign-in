# tauri-plugin-google-native-auth

A Tauri plugin for **Native Google Sign-In** on Android using the [Credential Manager API](https://developer.android.com/identity/credential-manager).

## Why this plugin?

On Android, Google [disallows](https://developers.google.com/identity/protocols/oauth2/native-app#disallowed-user-agents) OAuth authentication requests from WebViews. Since Tauri on Android relies on a WebView, traditional OAuth redirects will fail with a `403: disallowed_useragent` error.

This plugin bypasses the WebView by using the native Android Credential Manager API, providing a seamless and secure sign-in experience.

## Installation

### 1. Rust Dependency

Add the plugin to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-google-native-auth = { git = "https://github.com/2vil/tauri-plugin-google-native-auth" }
# Or via local path if you are developing
# tauri-plugin-google-native-auth = { path = "../plugins/tauri-plugin-google-native-auth" }
```

### 2. Register the Plugin

Update your `src-tauri/src/lib.rs`:

```rust
pub fn run() {
    let builder = tauri::Builder::default()
        // ... other plugins
        .plugin(tauri_plugin_opener::init());

    #[cfg(target_os = "android")]
    let builder = builder.plugin(tauri_plugin_google_native_auth::init());

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Usage (Frontend)

Call the plugin from your JavaScript/TypeScript code:

```ts
import { invoke } from "@tauri-apps/api/core";

export async function googleSignIn(webClientID: string) {
  try {
    const result = await invoke<{ success: boolean; data: string }>(
      "plugin:google-native-auth|signin",
      { payload: { client: webClientID } }
    );

    if (result.success) {
      // result.data contains the Google ID Token (JWT)
      return result.data;
    } else {
      throw new Error(result.data); // Error message
    }
  } catch (error) {
    console.error("Native Google Sign-In failed", error);
    throw error;
  }
}
```

## Google Cloud Console Configuration

To make this work, you need to configure your [Google Cloud Console](https://console.cloud.google.com/):

1.  **Web Client ID**: Create an OAuth 2.0 Client ID for "Web Application". This `client_id` is what you pass to the plugin. Your backend will use this to verify the token.
2.  **Android Client ID**: Create an OAuth 2.0 Client ID for "Android". You must provide your package name (e.g., `com.example.app`) and the **SHA-1 fingerprint** of your signing certificate (debug and/or release).

**Note**: You don't need to pass the Android Client ID to the plugin; Google handles the link automatically via the package name and SHA-1 fingerprint.

## Author

Created with ❤️ by **2vil**
