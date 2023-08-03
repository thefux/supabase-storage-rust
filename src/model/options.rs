use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub enum Resize {
    #[serde(rename = "cover")]
    Cover,
    #[serde(rename = "contain")]
    Contain,
    #[serde(rename = "fill")]
    Fill,
}

impl From<Resize> for &str {
    fn from(value: Resize) -> Self {
        match value {
            Resize::Cover => "cover",
            Resize::Contain => "contain",
            Resize::Fill => "fill",
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Format {
    #[serde(rename = "origin")]
    Origin,
    #[serde(rename = "avif")]
    Avif,
}

impl From<Format> for &str {
    fn from(value: Format) -> Self {
        match value {
            Format::Avif => "avif",
            Format::Origin => "origin",
        }
    }
}

/// * format: Specify the format of the image requested.
///           When using 'origin' we force the format to be the same as the original image.
///           When this option is not passed in, images are optimized to modern image formats like Webp.
/// * height: The height of the image in pixels.
/// * quality: Set the quality of the returned image.
///            A number from 20 to 100, with 100 being the highest quality.
///            Defaults to 80
/// * resize: The resize mode can be cover, contain or fill.
///           Defaults to cover.
///           Cover resizes the image to maintain it's aspect ratio while filling the entire width and height.
///           Contain resizes the image to maintain it's aspect ratio while fitting the entire image within the width and height. Fill resizes the image to fill the entire width and height. If the object's aspect ratio does not match the width and height, the image will be stretched to fit.
/// * width: The width of the image in pixels.
#[derive(Debug, Serialize)]
pub struct Transform {
    pub format: Option<Format>,
    pub height: Option<u32>,
    pub quality: Option<u32>,
    pub resize: Option<Resize>,
    pub width: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct Options {
    pub download: Option<bool>,
    pub transform: Option<Transform>,
}

/// * cache_control: The number of seconds the asset is cached in the browser and in the Supabase CDN.
///                  This is set in the `Cache-Control: max-age=<seconds>` header. Defaults to 3600 seconds
/// * content_type: the `Content-Type` header value.
///                 Should be specified if using a `fileBody` that is neither `Blob` nor `File` nor `FormData`,
///                 otherwise will default to `text/plain;charset=UTF-8`.
/// * upsert: When upsert is set to true, the file is overwritten if it exists.
///           When set to false, an error is thrown if the object already exists.
///           Defaults to false.
#[derive(Debug, Serialize)]
pub struct FileOptions {
    #[serde(serialize_with = "serialize_cache_control")]
    #[serde(rename = "cache-control")]
    pub cache_control: Option<u64>,
    #[serde(rename = "content-type")]
    pub content_type: Option<String>,
    pub upsert: Option<bool>,
}

fn serialize_cache_control<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(val) = value {
        serializer.serialize_str(&format!("max-age={}", val))
    } else {
        serializer.serialize_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_serialize_file_options() {
        let options = FileOptions {
            cache_control: Some(1000),
            content_type: Some("application/pdf".to_string()),
            upsert: Some(true),
        };
        let serialized = serde_json::to_string(&options).unwrap();
        assert_eq!(
            serialized,
            r#"{"cache-control":"max-age=1000","content-type":"application/pdf","upsert":true}"#
        );
    }
}
