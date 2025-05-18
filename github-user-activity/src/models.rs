#![allow(dead_code)]

use serde::Deserialize;

pub type Events = Vec<Event>;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Event {
    pub id: String,
    pub r#type: String,
    pub actor: Actor,
    pub repo: Repo,
    pub payload: Payload,
    pub public: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Actor {
    pub id: i64,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Payload {
    pub r#ref: Option<String>,
    pub ref_type: Option<String>,
    pub master_branch: Option<String>,
    pub description: Option<String>,
    pub pusher_type: Option<String>,
    pub action: Option<String>,
    pub repository_id: Option<i64>,
    pub push_id: Option<i64>,
    pub size: Option<i64>,
    pub distinct_size: Option<i64>,
    pub head: Option<String>,
    pub before: Option<String>,
    pub commits: Option<Vec<Commit>>,
    pub release: Option<Release>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Commit {
    pub sha: String,
    pub author: CommitAuthor,
    pub message: String,
    pub distinct: bool,
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CommitAuthor {
    pub email: String,
    pub name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i64,
    pub author: UploaderClass,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
    pub short_description_html: String,
    pub is_short_description_html_truncated: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: Option<serde_json::Value>,
    pub uploader: UploaderClass,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UploaderClass {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub user_view_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub url: String,
}
