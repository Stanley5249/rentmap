use base64::prelude::*;
use bytes::Bytes;
use miette::Diagnostic;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error, Diagnostic)]
pub enum DataUrlError {
    #[error("invalid data URL scheme {0:?}")]
    #[diagnostic(code(url::data::invalid_scheme), help("URL scheme must be 'data'"))]
    InvalidScheme(String),

    #[error("invalid data URL format {0:?}")]
    #[diagnostic(
        code(url::data::invalid_format),
        help("data URL must follow format: data:image/<ext>;base64,<data>")
    )]
    InvalidFormat(String),

    #[error("invalid media type {0:?}")]
    #[diagnostic(
        code(url::data::invalid_media_type),
        help("media type must be 'image'")
    )]
    InvalidMediaType(String),

    #[error("invalid encoding {0:?}")]
    #[diagnostic(code(url::data::invalid_encoding), help("encoding must be 'base64'"))]
    InvalidEncoding(String),

    #[error(transparent)]
    #[diagnostic(code(url::data::base64_decode_error))]
    Base64DecodeError(#[from] base64::DecodeError),
}

/// Decode the base64 data from a data URL and return the encoded string
/// For data URLs like "data:image/png;base64,iVBOR...", returns decoded bytes
pub fn decode_base64_image(data: &Url) -> Result<Bytes, DataUrlError> {
    if data.scheme() != "data" {
        return Err(DataUrlError::InvalidScheme(data.scheme().to_string()));
    }

    let path = data.path();

    let (media_type, temp) = path
        .split_once('/')
        .ok_or_else(|| DataUrlError::InvalidFormat(path.to_string()))?;

    let (_, temp) = temp
        .split_once(';')
        .ok_or_else(|| DataUrlError::InvalidFormat(path.to_string()))?;

    let (encoding, data) = temp
        .split_once(',')
        .ok_or_else(|| DataUrlError::InvalidFormat(path.to_string()))?;

    if media_type != "image" {
        return Err(DataUrlError::InvalidMediaType(media_type.to_string()));
    }

    if encoding != "base64" {
        return Err(DataUrlError::InvalidEncoding(encoding.to_string()));
    }

    let bytes = BASE64_STANDARD
        .decode(data)
        .map_err(DataUrlError::Base64DecodeError)?
        .into();

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_data_url() {
        let data_url = Url::parse("data:image/png;base64,aGVsbG8gd29ybGQ=").unwrap();
        let result = decode_base64_image(&data_url).unwrap();
        assert_eq!(result, "hello world".as_bytes());
    }

    #[test]
    fn test_invalid_scheme() {
        let url = Url::parse("https://example.com/image.png").unwrap();
        let result = decode_base64_image(&url);
        assert!(matches!(result, Err(DataUrlError::InvalidScheme(_))));
    }

    #[test]
    fn test_missing_slash_in_path() {
        let data_url = Url::parse("data:imagepng;base64,aGVsbG8=").unwrap();
        let result = decode_base64_image(&data_url);
        assert!(matches!(result, Err(DataUrlError::InvalidFormat(_))));
    }

    #[test]
    fn test_invalid_media_type() {
        let data_url = Url::parse("data:text/plain;base64,aGVsbG8=").unwrap();
        let result = decode_base64_image(&data_url);
        assert!(matches!(result, Err(DataUrlError::InvalidMediaType(_))));
    }

    #[test]
    fn test_missing_semicolon() {
        let data_url = Url::parse("data:image/png,aGVsbG8=").unwrap();
        let result = decode_base64_image(&data_url);
        assert!(matches!(result, Err(DataUrlError::InvalidFormat(_))));
    }

    #[test]
    fn test_missing_comma() {
        let data_url = Url::parse("data:image/png;base64").unwrap();
        let result = decode_base64_image(&data_url);
        assert!(matches!(result, Err(DataUrlError::InvalidFormat(_))));
    }

    #[test]
    fn test_invalid_encoding() {
        let data_url = Url::parse("data:image/png;charset=utf-8,hello").unwrap();
        let result = decode_base64_image(&data_url);
        assert!(matches!(result, Err(DataUrlError::InvalidEncoding(_))));
    }

    #[test]
    fn test_invalid_base64_data() {
        let data_url = Url::parse("data:image/png;base64,invalid!!!").unwrap();
        let result = decode_base64_image(&data_url);
        assert!(matches!(result, Err(DataUrlError::Base64DecodeError(_))));
    }

    #[test]
    fn test_empty_base64_data() {
        let data_url = Url::parse("data:image/png;base64,").unwrap();
        let result = decode_base64_image(&data_url).unwrap();
        assert_eq!(result, Bytes::new());
    }
}
