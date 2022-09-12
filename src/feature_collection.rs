use serde::{Deserialize, Serialize};

use crate::Item;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Context{
    pub limit: usize,
    pub matched: usize,
    pub returned: usize
}

/// The type field for [FeatureCollections](FeatureCollection).
pub const FEATURECOLLECTION_TYPE: &str = "FeatureCollection";

/// A FeatureCollection represents a stac query result.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FeatureCollection {
    /// Type of the GeoJSON Object. MUST be set to `"FeatureCollection"`.
    pub r#type: String,
    /// The STAC version the `Item` implements.
    #[serde(rename = "stac_version")]
    pub version: String,
    /// A list of extensions the `Item` implements.
    #[serde(rename = "stac_extensions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    pub context: Context,
    #[serde(rename = "features")]
    pub items: Vec<Item>,

}