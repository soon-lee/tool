use qiniu_sdk::upload::{
    apis::credential::Credential, AutoUploader, AutoUploaderObjectParams, UploadManager,
    UploadTokenSigner,
};
use reqwest::Client;
use serde_json::Value;
use std::{io::Cursor, sync::Arc, time::Duration};
use tokio_util::compat::TokioAsyncReadCompatExt;

pub(crate) struct OssClient {
    upload_manager: Arc<UploadManager>,
}
impl OssClient {
    pub fn from(access: &String, secret: &String, bucket: &String) -> Self {
        let credential = Credential::new(access, secret);
        let upload_manager = UploadManager::builder(UploadTokenSigner::new_credential_provider(
            credential,
            bucket,
            Duration::from_secs(3600),
        ))
        .build();
        Self {
            upload_manager: Arc::new(upload_manager),
        }
    }
    pub async fn post_bytes(&self, bytes: &[u8], object_name: &str) -> Result<Value, String> {
        let params = AutoUploaderObjectParams::builder()
            .object_name(object_name)
            .file_name(object_name)
            .build();
        let uploader: AutoUploader = self.upload_manager.auto_uploader();
        uploader
            .async_upload_reader(Cursor::new(Vec::from(bytes)).compat(), params)
            .await
            .map_err(|err| format!("上传失败：{}", err))
    }
    pub async fn post_from_url(&self, url: &String, name: &String) -> Result<Value, String> {
        let response = Client::new()
            .get(url)
            .send()
            .await
            .map_err(|err| format!("连接\"{}\"错误：{:?}", url, err))?;
        let bytes = response
            .bytes()
            .await
            .map_err(|err| format!("解析\"{}\"错误：{:?}", url, err))?;
        self.post_bytes(&bytes, name).await
    }
}
