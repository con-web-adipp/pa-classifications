use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BinaryCopy {
    #[serde(rename = "DisplayDirectory")]
    pub display_directory: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    #[serde(rename = "Sha1Hex")]
    pub sha1: String,
    #[serde(rename = "Files")]
    pub files: Vec<BinaryCopy>,
    #[serde(rename = "FileSize")]
    pub file_size: usize,
    #[serde(rename = "OutputDirectory")]
    pub output_directory: String,
    #[serde(rename = "Md5Hex")]
    pub md5: String,
}
