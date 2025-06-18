use crate::apis::vision::model::OcrString;
use bytes::Bytes;
use google_cloud_auth::credentials::api_key_credentials;
use google_cloud_vision_v1::client::ImageAnnotator;
use google_cloud_vision_v1::model::feature::Type;
use google_cloud_vision_v1::model::{
    AnnotateImageRequest, BatchAnnotateImagesRequest, Feature, Image, ImageContext,
};

/// Vision API client wrapper around google-cloud-vision-v1 with API key authentication
pub struct Client {
    client: ImageAnnotator,
}

impl Client {
    /// Create a new Vision API client with the provided API key
    pub async fn new(api_key: String) -> Result<Self, super::error::Error> {
        let credentials = api_key_credentials::Builder::new(api_key).build();

        let client = ImageAnnotator::builder()
            .with_credentials(credentials)
            .build()
            .await?;

        Ok(Client { client })
    }
    /// Perform text detection on multiple images (batch processing)
    /// Returns extracted text for all images
    pub async fn text_detection_batch<Images, Hints, B, S>(
        &self,
        images: Images,
        language_hints: Option<Hints>,
    ) -> Result<Vec<OcrString>, super::error::Error>
    where
        Images: IntoIterator<Item = B>,
        Hints: IntoIterator<Item = S>,
        B: Into<Bytes>,
        S: Into<String>,
    {
        let features = [Feature::new().set_type(Type::TextDetection)];

        let image_contents =
            language_hints.map(|hints| ImageContext::new().set_language_hints(hints));

        let requests: Vec<AnnotateImageRequest> = images
            .into_iter()
            .map(|content| {
                let image = Image::new().set_content(content);

                let mut request = AnnotateImageRequest::new()
                    .set_image(image)
                    .set_features(features.to_owned());

                request.image_context = image_contents.to_owned();

                request
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
            return Err(super::error::Error::ImageResponseCountMismatch {
                images: num_requests,
                responses: num_responses,
            });
        }

        let mut texts = Vec::new();

        for response in batch_response.responses {
            let text = response
                .full_text_annotation
                .ok_or(super::error::Error::TextAnnotationMissing)?
                .text
                .into();

            texts.push(text);
        }

        Ok(texts)
    }

    /// Perform text detection on a single image
    /// Returns extracted text for the image
    pub async fn text_detection_single<Hints, B, S>(
        &self,
        image: B,
        language_hints: Option<Hints>,
    ) -> Result<OcrString, super::error::Error>
    where
        Hints: IntoIterator<Item = S>,
        B: Into<Bytes>,
        S: Into<String>,
    {
        let result = self
            .text_detection_batch([image], language_hints)
            .await?
            .pop()
            .unwrap();

        Ok(result)
    }
}
