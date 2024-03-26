use std::collections::HashMap;
use std::env;
use std::process::exit;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use crate::define::USER_AGENT;

mod define;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a command");
    }
    match args[1].as_str() {
        "check-release" => check_release().await,
        "check-assets" => check_assets(&args).await,
        "upload-asset" => upload_asset(&args).await,
        not_match => panic!("Unknown command: {}", not_match),
    };
}

async fn load_envs() -> (String, String, String, String, String, Client) {
    let gh_token = std::env::var("GITHUB_TOKEN").unwrap();
    if gh_token.is_empty() {
        panic!("Please set GITHUB_TOKEN");
    }

    let repo = std::env::var("GITHUB_REPOSITORY").unwrap();
    if repo.is_empty() {
        panic!("Can't got repo path");
    }

    let branch = std::env::var("GITHUB_HEAD_REF").unwrap();
    if repo.is_empty() {
        panic!("Can't got repo branch");
    }

    let vs_code_txt = tokio::fs::read_to_string("version.code.txt").await.unwrap();
    let vs_info_txt = tokio::fs::read_to_string("version.info.txt").await.unwrap();

    let code = vs_code_txt.trim().to_owned();
    let info = vs_info_txt.trim().to_owned();

    let client = reqwest::ClientBuilder::new().user_agent(USER_AGENT).build().unwrap();

    return (
        gh_token,
        repo,
        branch,
        code,
        info,
        client
    );
}

async fn check_release() {
    println!("Checking release");
    let (
        gh_token,
        repo,
        branch,
        code,
        info,
        client
    ) = load_envs().await;
    let release_url = format!("https://api.github.com/repos/{repo}/releases/tags/{code}");
    let check_response = client.get(release_url).send().await.unwrap();
    match check_response.status().as_u16() {
        200 => {
            println!("release exists");
            exit(0);
        }
        404 => (),
        code => {
            let text = check_response.text().await.unwrap();
            panic!("error for check release : {} : {}", code, text);
        }
    }
    drop(check_response);

    // 404

    let releases_url = format!("https://api.github.com/repos/{repo}/releases");
    let check_response = client
        .post(releases_url)
        .header("Authorization", format!("token {}", gh_token))
        .json(&{
            let mut params = HashMap::<String, String>::new();
            params.insert("tag_name".to_string(), code.to_string());
            params.insert("target_commitish".to_string(), branch);
            params.insert("name".to_string(), code.to_string());
            params.insert("body".to_string(), info.to_string());
            params
        })
        .send()
        .await.unwrap();
    match check_response.status().as_u16() {
        201 => (),
        code => {
            let text = check_response.text().await.unwrap();
            panic!("error for create release : {} : {}", code, text);
        }
    }
}

async fn check_assets(args: &Vec<String>) {
    if args.len() < 4 {
        panic!("Please provide a release platform and arch");
    }
    let platform = args[2].as_str();
    let arch = args[3].as_str();
    let (
        _gh_token,
        repo,
        _branch,
        code,
        _info,
        client
    ) = load_envs().await;
    let assets_name = asset_name(
        repo.split("/").last().unwrap(),
        code.as_str(),
        platform,
        arch,
    );

    let check_response = client
        .get(format!(
            "https://api.github.com/repos/{repo}/releases/tags/{code}"
        ))
        .send()
        .await.unwrap();

    match check_response.status().as_u16() {
        200 => (),
        404 => println!("release not exists"),
        code => {
            let text = check_response.text().await.unwrap();
            panic!("error for check release : {} : {}", code, text);
        }
    }
    let release: Release = check_response.json().await.unwrap();

    let ass_names: Vec<String> = release.assets.iter().map(|a| a.name.clone()).collect();
    println!(
        "::set-output name=skip_build::{}",
        ass_names.contains(&assets_name)
    );
}

fn asset_name(app_name: &str, code: &str, platform: &str, arch: &str) -> String {
    let base_name = format!("{}-{}-{}-{}", app_name, code, platform, arch);
    let ext = match platform {
        "ios" => "ipa",
        "android" => "apk",
        _ => panic!("Unknown platform: {}", platform),
    };
    format!("{}.{}", base_name, ext)
}

async fn upload_asset(args: &Vec<String>) {
    if args.len() < 4 {
        panic!("Please provide a release platform and arch");
    }
    let platform = args[2].as_str();
    let arch = args[3].as_str();
    let (
        gh_token,
        repo,
        _branch,
        code,
        _info,
        client
    ) = load_envs().await;
    let assets_name = asset_name(
        repo.split("/").last().unwrap(),
        code.as_str(),
        platform,
        arch,
    );
    let local_path = match platform {
        "ios" => "../build/nosign.ipa",
        "android" => "../build/app/outputs/flutter-apk/app-release.apk",
        un => panic!("unknown target : {}", un),
    };


    let check_response = client
        .get(format!(
            "https://api.github.com/repos/{repo}/releases/tags/{code}"
        ))
        .header("Authorization", format!("token {}", gh_token))
        .send()
        .await.unwrap();

    match check_response.status().as_u16() {
        200 => (),
        404 => println!("release not exists"),
        code => {
            let text = check_response.text().await.unwrap();
            panic!("error for check release : {} : {}", code, text);
        }
    }
    let release: Release = check_response.json().await.unwrap();

    let buff = tokio::fs::read(local_path).await.unwrap();

    let response = client
        .post(format!(
            "https://uploads.github.com/repos/{}/releases/{}/assets?name={}",
            repo, release.id, assets_name
        ))
        .header("Authorization", format!("token {}", gh_token))
        .header("Content-Type", "application/octet-stream")
        .header("Content-Length", buff.len())
        .body(buff)
        .send()
        .await.unwrap();

    match response.status().as_u16() {
        201 => (),
        code => {
            let text = response.text().await.unwrap();
            panic!("error for check release : {} : {}", code, text);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i64,
    pub author: Author,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: Value,
    pub uploader: Uploader,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uploader {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}
