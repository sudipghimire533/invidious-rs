// Get given url and return it's response as bytes
pub async fn reqwest_get_bytes(url: url::Url) -> Result<Vec<u8>, reqwest::Error> {
    let get_res = reqwest::get(url).await?;
    let bytes = get_res.bytes().await?;
    Ok(bytes.to_vec())
}

crate::implement_all_api!(crate, InvidiousReqwest, reqwest::Error, reqwest_get_bytes);
