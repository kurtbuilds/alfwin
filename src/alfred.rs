/// Models for Alfred interop
use serde::Serialize;

#[derive(Serialize)]
pub enum AlfredItemType {
    #[serde(rename(serialize = "default"))]
    Default,
    #[serde(rename(serialize = "file"))]
    File,
    #[serde(rename(serialize = "file:skipcheck"))]
    FileSkipCheck,
}

#[derive(Serialize)]
pub enum ItemIconType {
    #[serde(rename(serialize = "path"))]
    FilePath,
    #[serde(rename(serialize = "fileicon"))]
    IconForFileAtPath,
}

#[derive(Serialize)]
pub struct ItemIcon {
    #[serde(rename(serialize = "type"))]
    pub typ: ItemIconType,
    pub path: String,
}

#[derive(Serialize)]
pub struct AlfredItem {
    pub uid: String,
    #[serde(rename(serialize = "type"))]
    pub typ: AlfredItemType,
    pub title: String,
    pub subtitle: String,
    pub arg: String,
    /// text to fill when you press Tab on the item
    pub autocomplete: String,
    pub icon: ItemIcon,
}

#[derive(Serialize)]
pub struct AlfredItems {
    pub items: Vec<AlfredItem>,
}
