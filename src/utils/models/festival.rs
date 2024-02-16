use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FestivalsResponse {
    pub data: FestivalData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FestivalData {
    pub all_festivals: Vec<Festival>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Festival {
    pub title: String,
    pub id: String,
    pub slug: String,
    pub address: Address,
    pub period: Vec<Period>,
    pub tags: Vec<Tag>,
    pub thumbnail: Thumbnail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub route: String,
    pub locality: String,
    pub administrative_area_level_3: String,
    pub administrative_area_level_2: String,
    pub administrative_area_level_1: String,
    pub country: String,
    pub postal_code: String,
    pub name: String,
    pub formatted_address: String,
    pub coordinates: Coordinates,
    pub utc_offset_minutes: i32,
    pub street_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    pub startdate: String,
    pub enddate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub responsive_image: ResponsiveImage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponsiveImage {
    pub src: String,
    pub src_set: String,
    pub width: u32,
    pub height: u32,
    pub alt: String,
    pub sizes: String,
}
