use aws_sdk_s3::{
    operation::{get_object::GetObjectError, put_object::PutObjectError},
    presigning::{PresignedRequest, PresigningConfig},
};

/// Get a presigned URL for uploading a file to S3
///
/// you can get the uri by calling [`uri()`] on the returned [`PresignedRequest`]
///
/// [`uri()`]: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/presigning/struct.PresignedRequest.html#method.uri
pub async fn get_presigned_download_url(
    client: &aws_sdk_s3::Client,
    object: &str,
    bucket: &str,
    expire_in: std::time::Duration,
) -> Result<PresignedRequest, GetObjectError> {
    client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(
            PresigningConfig::builder()
                .expires_in(expire_in)
                .build()
                .expect("Valid presigning config"),
        )
        .await
        .map_err(|e| e.into_service_error())
}

/// Get a presigned URL for uploading a file to S3
///
/// you can get the request by calling [`into_http_1x_request()`] on the returned [`PresignedRequest`]
///
/// [`into_http_1x_request()`]: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/presigning/struct.PresignedRequest.html#method.into_http_1x_request
pub async fn get_presigned_upload_url(
    client: &aws_sdk_s3::Client,
    object: &str,
    bucket: &str,
    expire_in: std::time::Duration,
) -> Result<PresignedRequest, PutObjectError> {
    client
        .put_object()
        .bucket(bucket)
        .key(object)
        .presigned(
            PresigningConfig::builder()
                .expires_in(expire_in)
                .build()
                .expect("Valid presigning config"),
        )
        .await
        .map_err(|e| e.into_service_error())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_get_presigned_download_url() {
        if dotenv::dotenv().is_err() {
            println!("Failed to read .env file");
        } else {
            println!(".env file loaded successfully");
        }
        let client = crate::s3::s3_client();
        let object = "test.txt";
        let bucket = "test-bucket";
        let expire_in = std::time::Duration::from_secs(60);

        let presigned_request = get_presigned_download_url(&client.await, object, bucket, expire_in)
            .await
            .expect("Get presigned download url");

        let uri = presigned_request.uri();
        assert!(uri.contains(object));
        assert!(uri.contains(bucket));
    }

    #[actix_web::test]
    async fn test_get_presigned_upload_url() {
        if dotenv::dotenv().is_err() {
            println!("Failed to read .env file");
        } else {
            println!(".env file loaded successfully");
        }
        let client = crate::s3::s3_client();
        let object = "test.txt";
        let bucket = "test-bucket";
        let expire_in = std::time::Duration::from_secs(60);

        let presigned_request = get_presigned_upload_url(&client.await, object, bucket, expire_in)
            .await
            .expect("Get presigned upload url");
        let uri = presigned_request.uri();
        assert!(uri.contains(object), "URI does not contain the object name");
        assert!(uri.contains(bucket), "URI does not contain the bucket name");

        let _ = presigned_request
            .into_http_1x_request(aws_sdk_s3::primitives::ByteStream::from(vec![]));
    }
}
