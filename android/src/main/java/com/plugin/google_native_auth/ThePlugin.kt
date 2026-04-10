package com.plugin.google_native_auth

import android.app.Activity
import android.util.Log

import androidx.credentials.CredentialManager
import androidx.credentials.GetCredentialRequest
import com.google.android.libraries.identity.googleid.GetGoogleIdOption
import com.google.android.libraries.identity.googleid.GoogleIdTokenCredential

import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.delay

@InvokeArg
class Args {
	var client: String = ""
}

@TauriPlugin
class ThePlugin(private val activity: Activity) : Plugin(activity) {
	@Command
	fun signin(invoke: Invoke) {
		val args = invoke.parseArgs(Args::class.java)

		CoroutineScope(Dispatchers.IO).launch {
			try {
				val result = signinAsync(args.client)
				invoke.resolve(result)
			} catch (e: Exception) {
				invoke.reject(e.message)
			}
		}
	}

	suspend fun signinAsync(clientId: String): JSObject {
		val credentialManager = CredentialManager.create(activity)

		val googleIdOption: GetGoogleIdOption = GetGoogleIdOption.Builder()
			.setFilterByAuthorizedAccounts(false)
			.setServerClientId(clientId)
			// Optional: add setNonce() for improved replay attack prevention
			// .setNonce("...")
			.build()

		val request: GetCredentialRequest = GetCredentialRequest.Builder()
			.addCredentialOption(googleIdOption)
			.build()

		val ret = JSObject()

		try {
			val result = credentialManager.getCredential(
				request = request,
				context = activity
			)
			if (result.credential.type != GoogleIdTokenCredential.TYPE_GOOGLE_ID_TOKEN_CREDENTIAL) {
				ret.put("success", false)
				ret.put("t",    result.credential.type)
				ret.put("data", "wrong credential type")
				return (ret)
			}

			try {
				val googleIdTokenCredential = GoogleIdTokenCredential.createFrom(result.credential.data)
				val idTokenString = googleIdTokenCredential.idToken

				ret.put("success", true)
				ret.put("t",    result.credential.type)
				ret.put("data", idTokenString)
			} catch (e: Exception) {
				Log.e("Auth", "Failed to parse Google ID Token: $e")
			}
		} catch (e: Exception) {
			ret.put("success", false)
			ret.put("t",    "")
			ret.put("data", e.message)
			// If 'NoCredentialsException' occurs, you often try again with setFilterByAuthorizedAccounts(false)
		}
		return (ret)
	}
}