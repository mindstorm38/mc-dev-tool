use isahc::prelude::*;
use serde_json::Value;
use std::collections::HashMap;


const MC_VERSION_MANIFEST: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";


pub fn request_versions() -> Option<MinecraftVersions> {

    let manifest: Value = isahc::get(MC_VERSION_MANIFEST)
        .unwrap()
        .json()
        .unwrap();

    let mut ret = MinecraftVersions::new();

    if let Value::Array(versions) = &manifest["versions"] {
        for version in versions {

            if let Value::Object(version) = version {

                if let Some(Value::String(id)) = version.get("id") {
                    if let Some(Value::String(url)) = version.get("url") {
                        if let Some(Value::String(typename)) = version.get("type") {
                            if let Some(typ) = MinecraftVersionType::from_name(typename) {
                                ret.add(id, typ, url);
                            }
                        }
                    }
                }

            }

        }
    }

    if let Value::Object(latest) = &manifest["latest"] {
        for (typename, version_raw) in latest {
            if let Value::String(version) = version_raw {
                if let Some(typ) = MinecraftVersionType::from_name(typename) {
                    ret.set_latest(typ, version);
                }
            }
        }
    }

    Some(ret)

}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MinecraftVersionType { Release, Snapshot, OldAlpha }

#[derive(Debug)]
pub struct MinecraftVersion {
    pub id: String,
    pub typ: MinecraftVersionType,
    pub url: String
}

#[derive(Debug)]
pub struct MinecraftVersions {
    versions: HashMap<String, MinecraftVersion>,
    latest: HashMap<MinecraftVersionType, String>,
    versions_order: Vec<String>
}

impl MinecraftVersionType {

    pub fn from_name(typ: &String) -> Option<MinecraftVersionType> {

        match typ.as_str() {
            "release" => Some(MinecraftVersionType::Release),
            "snapshot" => Some(MinecraftVersionType::Snapshot),
            "old_alpha" => Some(MinecraftVersionType::OldAlpha),
            _ => None
        }

    }

}

impl MinecraftVersions {

    pub fn new() -> MinecraftVersions {
        MinecraftVersions {
            versions: HashMap::new(),
            latest: HashMap::new(),
            versions_order: Vec::new()
        }
    }

    pub fn add(&mut self, id: &String, typ: MinecraftVersionType, url: &String) {

        self.versions.insert(id.clone(), MinecraftVersion {
            id: id.clone(),
            typ,
            url: url.clone()
        });

        self.versions_order.push(id.clone());

    }

    pub fn set_latest(&mut self, typ: MinecraftVersionType, id: &String) {
        self.latest.insert(typ, id.clone());
    }

    pub fn is_latest(&self, version: &MinecraftVersion) -> bool {
        match self.latest.get(&version.typ) {
            Some(id) => *id == version.id,
            None => false
        }
    }

    pub fn get_versions(&self) -> &Vec<String> {
        &self.versions_order
    }

    pub fn get_version(&self, id: &String) -> Option<&MinecraftVersion> {
        self.versions.get(id)
    }

}
