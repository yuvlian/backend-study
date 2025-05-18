#![allow(dead_code)]

use serde::Deserialize;

pub type Events = Vec<Event>;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Event {
    id: String,
    r#type: String,
    actor: Actor,
    repo: Repo,
    payload: Payload,
    public: bool,
    created_at: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Actor {
    id: i64,
    login: String,
    display_login: String,
    gravatar_id: String,
    url: String,
    avatar_url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Payload {
    r#ref: Option<String>,
    ref_type: Option<String>,
    master_branch: Option<String>,
    description: Option<String>,
    pusher_type: Option<String>,
    action: Option<String>,
    repository_id: Option<i64>,
    push_id: Option<i64>,
    size: Option<i64>,
    distinct_size: Option<i64>,
    head: Option<String>,
    before: Option<String>,
    commits: Option<Vec<Commit>>,
    release: Option<Release>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Commit {
    sha: String,
    author: CommitAuthor,
    message: String,
    distinct: bool,
    url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CommitAuthor {
    email: String,
    name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Release {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: i64,
    author: UploaderClass,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<Asset>,
    tarball_url: String,
    zipball_url: String,
    body: String,
    short_description_html: String,
    is_short_description_html_truncated: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Asset {
    url: String,
    id: i64,
    node_id: String,
    name: String,
    label: Option<serde_json::Value>,
    uploader: UploaderClass,
    content_type: String,
    state: String,
    size: i64,
    download_count: i64,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UploaderClass {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    user_view_type: String,
    site_admin: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Repo {
    id: i64,
    name: String,
    url: String,
}
