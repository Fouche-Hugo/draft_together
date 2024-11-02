use std::{
    io::Cursor,
    path::{Path, PathBuf},
};

use anyhow::Result;
use flate2::read::GzDecoder;
use semver::Version;
use tar::Archive;
use thiserror::Error;
use tracing::{debug, info, trace};

pub async fn get_latest_ddragon_version() -> Result<Version> {
    let response = reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json").await?;
    let versions: Vec<String> = response.json().await?;

    let last_version = versions
        .first()
        .expect("there should be at least one league of legends version");

    Ok(Version::parse(last_version)?)
}

pub const DATA_DRAGON_DIR: &str = "dragontail";

pub async fn get_ddragon_path_or_download(version: &Version) -> Result<PathBuf> {
    info!("start to download ddragon");

    let data_dragon_dir = Path::new(DATA_DRAGON_DIR);
    if !data_dragon_dir.exists() {
        debug!("data dragon dir: {DATA_DRAGON_DIR} not found, creating it");
        std::fs::create_dir(data_dragon_dir)?;
    }

    let file_path = PathBuf::from(format!("{DATA_DRAGON_DIR}/dragontail-{version}.tgz"));
    if !file_path.exists() {
        let url = format!("https://ddragon.leagueoflegends.com/cdn/dragontail-{version}.tgz");
        let response = reqwest::get(url).await?;
        let mut content = Cursor::new(response.bytes().await?);
        let mut file = std::fs::File::create(&file_path)?;
        std::io::copy(&mut content, &mut file)?;
    }

    Ok(file_path)
}

pub fn decompress_tarball(
    tarball_path: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<()> {
    let tar_gz = std::fs::File::open(tarball_path.as_ref())?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    archive.unpack(output_dir)?;

    Ok(())
}

const CHAMPION_FULL_FILENAME: &str = "championFull.json";
const DATA_DRAGON_CHAMPION_FULL_PATH: &str = "/data/en_US/championFull.json";
const DATA_DRAGON_IMAGE_PATH: &str = "img/champion";
const DATA_DRAGON_CENTERED_IMAGE_PATH: &str = "img/champion/centered";
const SUB_DIRECTORY_IMAGE_PATH: &str = "img";

pub fn extract_data_from_ddragon(
    ddragon_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    version: &Version,
) -> Result<Vec<ChampionDataDragon>> {
    let ddragon_path = ddragon_path.as_ref();
    let champions_json_path =
        ddragon_path.join(format!("{version}{DATA_DRAGON_CHAMPION_FULL_PATH}"));
    info!(?champions_json_path);

    let output_path = output_path.as_ref();
    if !output_path.exists() {
        std::fs::create_dir_all(output_path)?;
    }
    let image_output_path = output_path.join(SUB_DIRECTORY_IMAGE_PATH);
    if !image_output_path.exists() {
        std::fs::create_dir_all(&image_output_path)?;
    }

    let output_path_champion_json = output_path.join(CHAMPION_FULL_FILENAME);
    std::fs::copy(champions_json_path, &output_path_champion_json)?;
    let mut champions_riot = parse_champion_json(output_path_champion_json)?;

    for champion in &mut champions_riot {
        let champion_centered_path = ddragon_path
            .join(DATA_DRAGON_CENTERED_IMAGE_PATH)
            .join(&champion.centered_default_skin_image_path);
        let output_centered_path =
            image_output_path.join(&champion.centered_default_skin_image_path);

        if champion.riot_id == "Fiddlesticks" {
            correct_fiddlesticks_image_name(&champion_centered_path)?;
        }

        let output_default_image_path = image_output_path.join(&champion.default_skin_image_path);

        std::fs::copy(&champion_centered_path, &output_centered_path)?;
        std::fs::copy(
            ddragon_path
                .join(version.to_string())
                .join(DATA_DRAGON_IMAGE_PATH)
                .join(&champion.default_skin_image_path),
            &output_default_image_path,
        )?;

        champion.default_skin_image_path = output_default_image_path.to_string_lossy().to_string();
        champion.centered_default_skin_image_path =
            output_centered_path.to_string_lossy().to_string();
    }

    Ok(champions_riot)
}

fn correct_fiddlesticks_image_name(path_from_data_dragon: impl AsRef<Path>) -> Result<()> {
    let path_from_data_dragon = path_from_data_dragon.as_ref();

    if !path_from_data_dragon.exists() {
        // Try to replace "Fiddlesticks" by "FiddleSticks" due to riot inconsistency
        let current_path = path_from_data_dragon
            .to_str()
            .expect("riot filenames should be utf-8")
            .replace("Fiddlesticks", "FiddleSticks");
        let current_path = Path::new(&current_path);

        if current_path.exists() {
            debug!(
                "fixing Fiddlesticks error, renaming {current_path:?} to {path_from_data_dragon:?}"
            );
            std::fs::rename(current_path, path_from_data_dragon)?;
        }
    }

    Ok(())
}

#[derive(Error, Debug, Clone)]
enum ChampionJsonParsingError {
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("field {field} is not of type {expected_type}")]
    WrongTypeField {
        field: &'static str,
        expected_type: &'static str,
    },
}

#[derive(Debug, Clone)]
pub struct ChampionDataDragon {
    pub riot_id: String,
    pub name: String,
    pub default_skin_image_path: String,
    pub centered_default_skin_image_path: String,
}

fn parse_champion_json(path_champion_json: impl AsRef<Path>) -> Result<Vec<ChampionDataDragon>> {
    info!(
        "Parsing champion_json file: {:?}",
        path_champion_json.as_ref()
    );
    let champions: serde_json::Value =
        serde_json::from_reader(std::fs::File::open(path_champion_json)?)?;

    let champions = parse_field_as_object(&champions, "data")?;

    let mut champions_riot_data = Vec::with_capacity(champions.len());
    for (field, data) in champions {
        debug!("Parsing data from {field}");

        let id = parse_field_as_str(data, "id")?;
        trace!("{field} champion id: {id}");

        let name = parse_field_as_str(data, "name")?;
        trace!("{field} champion name: {name}");

        let image = parse_field_as_object(data, "image")?;
        trace!("{field} champion image object: {image:?}");

        let image_full = image
            .get("full")
            .ok_or(ChampionJsonParsingError::MissingField("full"))?
            .as_str()
            .ok_or(ChampionJsonParsingError::WrongTypeField {
                field: "full",
                expected_type: "str",
            })?;
        trace!("{field} champion image full: {image_full}");

        let skins = parse_field_as_array(data, "skins")?;
        trace!("{field} champion skins: {skins:?}");
        let default_skin = skins.iter().find(|skin| {
            let Ok(name_field) = parse_field_as_str(skin, "name") else {
                return false;
            };

            name_field == "default"
        });

        let default_skin_num = default_skin.map(|skin| parse_field_as_i64(skin, "num"));
        let default_skin_num = default_skin_num.unwrap_or(Ok(0)).unwrap_or(0);
        trace!("{field} champion default skin num: {default_skin_num}");

        champions_riot_data.push(ChampionDataDragon {
            riot_id: id.to_string(),
            centered_default_skin_image_path: format!("{id}_{default_skin_num}.jpg"),
            default_skin_image_path: image_full.to_string(),
            name: name.to_string(),
        })
    }

    Ok(champions_riot_data)
}

fn parse_field_as_str<'a>(
    json_data: &'a serde_json::Value,
    field: &'static str,
) -> Result<&'a str> {
    json_data
        .get(field)
        .ok_or(ChampionJsonParsingError::MissingField(field))
        .map(|value| value.as_str())?
        .ok_or(
            ChampionJsonParsingError::WrongTypeField {
                field,
                expected_type: "str",
            }
            .into(),
        )
}

fn parse_field_as_i64(json_data: &serde_json::Value, field: &'static str) -> Result<i64> {
    json_data
        .get(field)
        .ok_or(ChampionJsonParsingError::MissingField(field))
        .map(|value| value.as_i64())?
        .ok_or(
            ChampionJsonParsingError::WrongTypeField {
                field,
                expected_type: "i64",
            }
            .into(),
        )
}

fn parse_field_as_object<'a>(
    json_data: &'a serde_json::Value,
    field: &'static str,
) -> Result<&'a serde_json::Map<String, serde_json::Value>> {
    json_data
        .get(field)
        .ok_or(ChampionJsonParsingError::MissingField(field))
        .map(|value| value.as_object())?
        .ok_or(
            ChampionJsonParsingError::WrongTypeField {
                field,
                expected_type: "object",
            }
            .into(),
        )
}

fn parse_field_as_array<'a>(
    json_data: &'a serde_json::Value,
    field: &'static str,
) -> Result<&'a Vec<serde_json::Value>> {
    json_data
        .get(field)
        .ok_or(ChampionJsonParsingError::MissingField(field))
        .map(|value| value.as_array())?
        .ok_or(
            ChampionJsonParsingError::WrongTypeField {
                field,
                expected_type: "object",
            }
            .into(),
        )
}
