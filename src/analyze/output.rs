use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CustomProperty {
    #[serde(rename = "Id")]
    pub id: usize,
    #[serde(rename = "Sha1")]
    pub sha1: String,
    #[serde(rename = "Value")]
    pub value: String,
}

impl CustomProperty {
    #[allow(dead_code)]
    pub fn new(id: usize, sha1: &str, value: &str) -> CustomProperty {
        Self {
            id,
            sha1: sha1.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Bookmark {
    #[serde(rename = "BookmarkPath")]
    pub path: String,
    #[serde(rename = "HtmlColor")]
    pub html_color: String,
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "Sha1")]
    pub sha1: String,
}

impl Bookmark {
    pub fn new(sha1: &str, path: &str) -> Self {
        Self {
            path: path.to_string(),
            html_color: "#FFFFFF".to_string(),
            comment: Default::default(),
            sha1: sha1.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn with_color(mut self, html_color: &str) -> Self {
        self.html_color = html_color.to_string();
        self
    }
    pub fn with_comment(mut self, comment: &str) -> Self {
        self.comment = comment.to_string();
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Output {
    #[serde(rename = "CustomProperties")]
    pub custom_properties: Vec<CustomProperty>,
    #[serde(rename = "Bookmarks")]
    pub bookmarks: Vec<Bookmark>,
}
