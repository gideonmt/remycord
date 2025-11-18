use anyhow::{Context, Result};

#[cfg(target_os = "windows")]
pub async fn get_token() -> Result<String> {
    use windows::Win32::Security::Credentials::{
        CredReadW, CredFree, CREDENTIALW, CRED_TYPE_GENERIC,
    };
    use windows::core::PWSTR;
    
    unsafe {
        let target = "remycord\0".encode_utf16().collect::<Vec<u16>>();
        let mut credential: *mut CREDENTIALW = std::ptr::null_mut();
        
        CredReadW(
            PWSTR(target.as_ptr() as *mut u16),
            CRED_TYPE_GENERIC,
            0,
            &mut credential,
        )
        .context("Failed to read credential from Windows Credential Manager")?;
        
        if credential.is_null() {
            anyhow::bail!("Token not found in Windows Credential Manager");
        }
        
        let cred = &*credential;
        let password_slice = std::slice::from_raw_parts(
            cred.CredentialBlob as *const u8,
            cred.CredentialBlobSize as usize,
        );
        let token = String::from_utf8(password_slice.to_vec())
            .context("Invalid UTF-8 in stored token")?;
        
        CredFree(credential as *const _);
        
        Ok(token)
    }
}

#[cfg(target_os = "macos")]
pub async fn get_token() -> Result<String> {
    use security_framework::passwords::get_generic_password;
    
    let password = get_generic_password("remycord", "token")
        .context("Failed to read token from macOS Keychain")?;
    
    String::from_utf8(password).context("Invalid UTF-8 in stored token")
}

#[cfg(target_os = "linux")]
pub async fn get_token() -> Result<String> {
    use secret_service::SecretService;
    use secret_service::EncryptionType;
    use std::collections::HashMap;
    
    let ss = SecretService::connect(EncryptionType::Dh)
        .await
        .context("Failed to connect to Secret Service")?;
    
    let collection = ss.get_default_collection()
        .await
        .context("Failed to get default collection")?;
    
    let mut search_attributes = HashMap::new();
    search_attributes.insert("service", "remycord");
    search_attributes.insert("username", "token");
    
    let items = collection.search_items(search_attributes)
        .await
        .context("Failed to search for token")?;
    
    if items.is_empty() {
        anyhow::bail!("Token not found in keyring");
    }
    
    let item = &items[0];
    let secret = item.get_secret()
        .await
        .context("Failed to get secret from keyring")?;
    
    String::from_utf8(secret).context("Invalid UTF-8 in stored token")
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub async fn get_token() -> Result<String> {
    anyhow::bail!("Token storage not supported on this platform")
}
