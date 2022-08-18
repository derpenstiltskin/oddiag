use anyhow::Result;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug)]
#[allow(unused)]
pub struct ClientAccount {
    tenant_id: String,
    tenant_name: String,
    username: String,
    path: String,
}

#[allow(unused)]
impl ClientAccount {
    pub fn new(tenant_id: String, tenant_name: String, username: String, path: String) -> ClientAccount {
        ClientAccount {
            tenant_id,
            tenant_name,
            username,
            path,
        }
    }

    pub fn get_tenant_id(&self) -> &String {
        &self.tenant_id
    }

    pub fn get_tenant_name(&self) -> &String {
        &self.tenant_name
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Client {
    version: String,
    install_path: String,
    business_accounts: Vec<ClientAccount>,
}

#[allow(unused)]
impl Client {
    pub fn new() -> Client {
        Client {
            version: String::new(),
            install_path: String::new(),
            business_accounts: Vec::new(),
        }
    }
    
    pub fn scan(&mut self, username: Option<&String>) -> Result<()> {
        let client_key_path = "Software\\Microsoft\\OneDrive";
        let client_accounts_key_path = "Software\\Microsoft\\OneDrive\\Accounts";

        let client_key = RegKey::predef(HKEY_CURRENT_USER).open_subkey(&client_key_path)?;
        let client_version: String = client_key.get_value("Version")?;
        let client_install_path: String = client_key.get_value("CurrentVersionPath")?;

        let client_accounts_key = RegKey::predef(HKEY_CURRENT_USER).open_subkey(&client_accounts_key_path)?;
        for client_account in client_accounts_key.enum_keys().map(|a| a.unwrap()).filter(|a| a.starts_with("Business")) {
            let client_account_key_path = format!("{}\\{}", &client_accounts_key_path, &client_account);

            let client_account_key = RegKey::predef(HKEY_CURRENT_USER).open_subkey(&client_account_key_path)?;
            let client_account_tenant_id: String = client_account_key.get_value("ConfiguredTenantId")?;
            let client_account_tenant_name: String = client_account_key.get_value("DisplayName")?;
            let client_account_username: String = client_account_key.get_value("UserEmail")?;
            let client_account_path: String = client_account_key.get_value("UserFolder")?;

            if let Some(u) = username {
                if client_account_username.eq(u) {
                    self.business_accounts.push(ClientAccount::new(client_account_tenant_id, client_account_tenant_name, client_account_username, client_account_path))
                }
            } else {
                self.business_accounts.push(ClientAccount::new(client_account_tenant_id, client_account_tenant_name, client_account_username, client_account_path))
            }
        }

        self.version.push_str(client_version.as_str());
        self.install_path.push_str(client_install_path.as_str());

        Ok(())
    }


    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn get_install_path(&self) -> &String {
        &self.install_path
    }

    pub fn get_business_accounts(&self) -> &Vec<ClientAccount> {
        &self.business_accounts
    }

    pub fn fix_hidden_login(&self) -> Result<()> {
        let (presign_key, disp) = RegKey::predef(HKEY_CURRENT_USER).create_subkey_with_flags("Software\\Microsoft\\OneDrive\\PreSignInRampOverrides", KEY_ALL_ACCESS)?;
        presign_key.set_value("1559", &0u32)?;

        Ok(())
    }

    pub fn enable_health_reporting(&self) -> Result<()> {
        let (onedrive_key, disp) = RegKey::predef(HKEY_LOCAL_MACHINE).create_subkey_with_flags("Software\\Policies\\Microsoft\\OneDrive", KEY_ALL_ACCESS)?;
        onedrive_key.set_value("EnableSyncAdminReports", &1u32)?;

        Ok(())
    }

    pub fn disable_health_reporting(&self) -> Result<()> {
        let (onedrive_key, disp) = RegKey::predef(HKEY_LOCAL_MACHINE).create_subkey_with_flags("Software\\Policies\\Microsoft\\OneDrive", KEY_ALL_ACCESS)?;
        onedrive_key.set_value("EnableSyncAdminReports", &0u32)?;
        
        Ok(())
    }
}