use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;

pub struct GraphClient {
    tenant_id: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    bearer_token: Option<String>,
    http: reqwest::Client,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GraphPage {
    value: Vec<Value>,
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
}

impl GraphClient {
    /// App-only auth: exchanges an app registration's client credentials for a
    /// token on every `authenticate()` call.
    pub fn new(tenant_id: String, client_id: String, client_secret: String) -> Self {
        Self {
            tenant_id: Some(tenant_id),
            client_id: Some(client_id),
            client_secret: Some(client_secret),
            bearer_token: None,
            http: reqwest::Client::new(),
        }
    }

    /// Bring-your-own-token: reuses an already-issued (e.g. delegated) Graph
    /// access token instead of performing a client-credentials exchange. No
    /// app registration / client secret required.
    pub fn from_token(access_token: String) -> Self {
        Self {
            tenant_id: None,
            client_id: None,
            client_secret: None,
            bearer_token: Some(access_token),
            http: reqwest::Client::new(),
        }
    }

    async fn token(&self) -> Result<String> {
        let tenant_id = self
            .tenant_id
            .as_deref()
            .context("tenant_id is required for client-credentials auth")?;
        let client_id = self
            .client_id
            .as_deref()
            .context("client_id is required for client-credentials auth")?;
        let client_secret = self
            .client_secret
            .as_deref()
            .context("client_secret is required for client-credentials auth")?;
        let url = format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token");
        let resp = self
            .http
            .post(&url)
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", client_id),
                ("client_secret", client_secret),
                ("scope", "https://graph.microsoft.com/.default"),
            ])
            .send()
            .await
            .context("token request failed")?
            .error_for_status()
            .context("token endpoint returned error")?
            .json::<TokenResponse>()
            .await
            .context("failed to parse token response")?;
        Ok(resp.access_token)
    }

    async fn get_all_pages(&self, token: &str, url: &str) -> Result<Vec<Value>> {
        let mut results = Vec::new();
        let mut next = Some(url.to_string());

        while let Some(current_url) = next {
            let page = self
                .http
                .get(&current_url)
                .bearer_auth(token)
                .header("ConsistencyLevel", "eventual")
                .send()
                .await
                .context("Graph API request failed")?
                .error_for_status()
                .context("Graph API returned error")?
                .json::<GraphPage>()
                .await
                .context("failed to parse Graph API response")?;

            results.extend(page.value);
            next = page.next_link;
        }
        Ok(results)
    }

    pub async fn get_users(&self, token: &str) -> Result<Vec<Value>> {
        self.get_all_pages(
            token,
            "https://graph.microsoft.com/v1.0/users?$select=id,displayName,userPrincipalName,accountEnabled",
        )
        .await
    }

    pub async fn get_groups(&self, token: &str) -> Result<Vec<Value>> {
        self.get_all_pages(
            token,
            "https://graph.microsoft.com/v1.0/groups?$select=id,displayName,groupTypes",
        )
        .await
    }

    pub async fn get_group_members(&self, token: &str, group_id: &str) -> Result<Vec<Value>> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/groups/{group_id}/members?$select=id,displayName,@odata.type"
        );
        self.get_all_pages(token, &url).await
    }

    pub async fn get_directory_roles(&self, token: &str) -> Result<Vec<Value>> {
        self.get_all_pages(
            token,
            "https://graph.microsoft.com/v1.0/directoryRoles?$select=id,displayName,roleTemplateId",
        )
        .await
    }

    pub async fn get_directory_role_members(
        &self,
        token: &str,
        role_id: &str,
    ) -> Result<Vec<Value>> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/directoryRoles/{role_id}/members?$select=id,displayName,@odata.type"
        );
        self.get_all_pages(token, &url).await
    }

    pub async fn get_applications(&self, token: &str) -> Result<Vec<Value>> {
        self.get_all_pages(
            token,
            "https://graph.microsoft.com/v1.0/applications?$select=id,displayName,appId,requiredResourceAccess",
        )
        .await
    }

    pub async fn get_service_principals(&self, token: &str) -> Result<Vec<Value>> {
        self.get_all_pages(
            token,
            "https://graph.microsoft.com/v1.0/servicePrincipals?$select=id,displayName,appId,appRoles,oauth2PermissionScopes",
        )
        .await
    }

    pub async fn get_sp_app_role_assignments(
        &self,
        token: &str,
        sp_id: &str,
    ) -> Result<Vec<Value>> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/servicePrincipals/{sp_id}/appRoleAssignments"
        );
        self.get_all_pages(token, &url).await
    }

    pub async fn get_oauth2_permission_grants(&self, token: &str) -> Result<Vec<Value>> {
        self.get_all_pages(
            token,
            "https://graph.microsoft.com/v1.0/oauth2PermissionGrants",
        )
        .await
    }

    pub async fn authenticate(&self) -> Result<String> {
        if let Some(token) = &self.bearer_token {
            return Ok(token.clone());
        }
        self.token().await
    }
}
