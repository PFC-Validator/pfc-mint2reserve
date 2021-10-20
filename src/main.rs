mod errors;
mod messages;

use anyhow::Result;

use crate::messages::{NFTMeta, NewNFTRequest};
use dotenv::dotenv;
use secp256k1::{All, Secp256k1};
use std::path::Path;
use std::{env, fs};
use structopt::StructOpt;
use terra_rust_api::PrivateKey;

/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[derive(StructOpt)]
struct Cli {
    #[structopt(name = "meta-directory", help = "meta-data directory")]
    meta_directory: String,
}

async fn run() -> Result<()> {
    let cli: Cli = Cli::from_args();
    let secp: Secp256k1<All> = Secp256k1::new();
    let signing_key_phrase =
        env::var("RESERVATION_AUTH").expect("Environment Variable 'RESERVATION_AUTH' Not present");
    let reservation_nft_url = env::var("RESERVATION_NFT_URL")
        .expect("Environment Variable 'RESERVATION_NFT_URL' Not present");

    let signing_key = PrivateKey::from_words(&secp, &signing_key_phrase).unwrap();
    let dir = Path::new(&cli.meta_directory);
    if !dir.is_dir() {
        eprintln!("Expected a directory")
    } else {
        for meta_file_r in fs::read_dir(dir)? {
            let meta_file = meta_file_r?;
            if let Some(name) = meta_file.file_name().to_str() {
                if name.ends_with(".json") {
                    let meta_file_name = meta_file.path();
                    // println!("name {:?}", meta_file_name);
                    let tp = NFTMeta::read_meta(&meta_file_name)?;
                    let tp_json = serde_json::to_string(&tp)?;
                    let new_nft = NewNFTRequest {
                        name: tp.name.clone(),
                        meta: tp_json.to_string(),
                        svg: "{}".to_string(),
                        ipfs_image: tp.image,
                        ipfs_meta: tp.token_uri,
                        image_data: tp.image_data,
                        external_url: tp.external_url,
                        description: tp.description,
                        background_color: tp.background_color,
                        animation_url: tp.animation_url,
                        youtube_url: tp.youtube_url,
                    };
                    let new_nft_json = serde_json::to_string(&new_nft)?;
                    let sig = signing_key.sign(&secp, &new_nft_json).unwrap();
                    //     println!("Message:\n{}", new_nft_json);
                    //     println!("Signature:\n{}", sig.signature);
                    //     println!("Public Key:\n{}", sig.pub_key.value);
                    let client = reqwest::Client::new();
                    let res = client
                        .post(&reservation_nft_url)
                        .header("X-Reservation-Signature", sig.signature)
                        .json(&new_nft)
                        .send()
                        .await?;
                    let status = res.status();
                    //    let resp = String::from_utf8_lossy(&res.bytes().await?);
                    if !status.is_success() {
                        log::info!("res - {} {} {}", status, &tp.name, name);
                    }
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    if let Err(ref err) = run().await {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        ::std::process::exit(1);
    }
}
