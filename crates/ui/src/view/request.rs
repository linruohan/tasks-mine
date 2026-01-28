use std::collections::HashMap;

use gpui::{App, AppContext, Context, Entity, Window};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct RequestClient {
    cookies: HashMap<String, Cookie>,
}

impl RequestClient {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { cookies: HashMap::new() }
    }

    pub fn add_cookie(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }

    pub fn remove_cookie(&mut self, name: &str) {
        self.cookies.remove(name);
    }

    pub fn get_cookie(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }

    pub fn clear_cookies(&mut self) {
        self.cookies.clear();
    }

    pub async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;

        let response = client.get(url).send().await?;
        let body = response.text().await?;

        Ok(body)
    }

    pub async fn post(
        &self,
        url: &str,
        body: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;

        let response =
            client.post(url).header("Content-Type", "application/json").body(body).send().await?;

        let response_body = response.text().await?;

        Ok(response_body)
    }

    pub async fn login(
        &mut self,
        url: &str,
        username: &str,
        password: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;

        let body = serde_json::json!({
            "username": username,
            "password": password,
        });

        let response =
            client.post(url).header("Content-Type", "application/json").json(&body).send().await?;

        // 保存 cookies
        if let Some(cookie_store) = response.cookies().next() {
            let cookie = Cookie {
                name: cookie_store.name().to_string(),
                value: cookie_store.value().to_string(),
                domain: cookie_store.domain().unwrap_or("").to_string(),
                path: cookie_store.path().unwrap_or("/").to_string(),
            };
            self.add_cookie(cookie);
        }

        Ok(response.status().is_success())
    }

    pub async fn fetch_dts_issues(
        &self,
        url: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let body = self.get(url).await?;
        let issues: Vec<serde_json::Value> = serde_json::from_str(&body)?;
        Ok(issues)
    }

    pub async fn fetch_merge_requests(
        &self,
        url: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let body = self.get(url).await?;
        let mrs: Vec<serde_json::Value> = serde_json::from_str(&body)?;
        Ok(mrs)
    }

    pub async fn fetch_requirements(
        &self,
        url: &str,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let body = self.get(url).await?;
        let reqs: Vec<serde_json::Value> = serde_json::from_str(&body)?;
        Ok(reqs)
    }
}
