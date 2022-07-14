use lazy_static::lazy_static;
use reqwest::{Client, Url};
use std::{collections::BTreeMap, path::Path};
use urlencoding::encode;

pub const CACHE_PATH: &str = "target/fetch_cache";

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

pub async fn api_get(
    base: &str,
    path: &str,
    parameters: BTreeMap<String, String>,
) -> eyre::Result<String> {
    let mut file_path = Path::new(CACHE_PATH).join(path);
    for (k, v) in &parameters {
        file_path = file_path.join(encode(k).as_ref()).join(encode(v).as_ref());
    }

    if file_path.is_file() {
        return Ok(std::fs::read_to_string(&file_path)?);
    }

    let mut url = Url::parse(&format!("{}{}", base, path))?;
    {
        let mut query = url.query_pairs_mut();
        for (k, v) in &parameters {
            query.append_pair(k.as_str(), v.as_str());
        }
    }

    let text = CLIENT.get(url).send().await?.text().await?;

    let _ = std::fs::create_dir_all(file_path.parent().unwrap());
    std::fs::write(file_path, &text)?;

    Ok(text)
}
