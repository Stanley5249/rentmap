use bytes::Bytes;
use google_cloud_auth::credentials::api_key_credentials;
use google_cloud_vision_v1::client::ImageAnnotator;
use google_cloud_vision_v1::model::feature::Type;
use google_cloud_vision_v1::model::{
    AnnotateImageRequest, BatchAnnotateImagesRequest, Feature, Image, ImageContext,
};

use super::error::Error;

/// Vision API client wrapper around google-cloud-vision-v1 with API key authentication
pub struct Client {
    pub client: ImageAnnotator,
    pub language_hints: Option<Vec<String>>,
}

impl Client {
    pub fn new(client: ImageAnnotator) -> Self {
        Self {
            client,
            language_hints: None,
        }
    }

    /// Create a new Vision API client with the provided API key
    pub async fn from_api_key(api_key: String) -> Result<Self, Error> {
        let credentials = api_key_credentials::Builder::new(api_key).build();

        let client = ImageAnnotator::builder()
            .with_credentials(credentials)
            .build()
            .await?;

        Ok(Client::new(client))
    }

    /// Set language hints for OCR requests
    pub fn with_language_hints(mut self, hints: Vec<String>) -> Self {
        self.language_hints = Some(hints);
        self
    }

    /// Perform text detection on multiple images (batch processing)
    /// Returns extracted text for all images
    pub async fn text_detection_batch<I, T>(&self, images: I) -> Result<Vec<String>, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<Bytes>,
    {
        let features = [Feature::new().set_type(Type::TextDetection)];

        let image_context = self
            .language_hints
            .as_ref()
            .map(|hints| ImageContext::new().set_language_hints(hints));

        let requests: Vec<_> = images
            .into_iter()
            .map(|content| {
                AnnotateImageRequest::new()
                    .set_image(Image::new().set_content(content))
                    .set_features(features.clone())
                    .set_or_clear_image_context(image_context.clone())
            })
            .collect();

        let num_requests = requests.len();

        let batch_request: BatchAnnotateImagesRequest =
            BatchAnnotateImagesRequest::new().set_requests(requests);

        let batch_response = self
            .client
            .batch_annotate_images()
            .with_request(batch_request)
            .send()
            .await?;

        let num_responses = batch_response.responses.len();

        if num_requests != num_responses {
            return Err(Error::CountMismatch {
                images: num_requests,
                responses: num_responses,
            });
        }

        let texts = batch_response
            .responses
            .into_iter()
            .map(|response| {
                response
                    .full_text_annotation
                    .map(|annotation| annotation.text)
                    .ok_or(Error::NoText)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(texts)
    }

    /// Perform text detection on a single image
    /// Returns extracted text for the image
    pub async fn text_detection_single<T>(&self, image: T) -> Result<String, Error>
    where
        T: Into<Bytes>,
    {
        let result = self.text_detection_batch([image]).await?.pop().unwrap();

        Ok(result)
    }
}
