use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};

use crate::{
    dto::host::{CurrentHostDto, RawHost},
    model::host_group::HostGroupModel,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct HostGroupDto {
    pub host_group_name: String,
    pub hosts: Vec<CurrentHostDto>,
}

impl HostGroupDto {
    pub fn new(host_group_name: String, hosts: Vec<CurrentHostDto>) -> Self {
        Self {
            host_group_name,
            hosts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateHostGroupsDto(pub Vec<HostGroupDto>);

impl From<HostGroupModel> for HostGroupDto {
    fn from(
        HostGroupModel {
            host_group_name,
            host_models,
        }: HostGroupModel,
    ) -> Self {
        Self {
            host_group_name,
            hosts: host_models
                .into_iter()
                .map(|el| CurrentHostDto::from(el))
                .collect(),
        }
    }
}
impl Serialize for CreateHostGroupsDto {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        // let mut map: HashMap<String, Vec<RawHost>> = HashMap::new();
        let mut map = s.serialize_map(Some(self.0.len()))?;
        for group in self.0.iter() {
            let g_name = group.host_group_name.clone();
            let raw_host_list = group
                .hosts
                .iter()
                .map(|h| RawHost {
                    host_name: h.host_name.clone(),
                    host_url: h.host_url.clone(),
                })
                .collect::<Vec<_>>();
            map.serialize_entry(&g_name, &raw_host_list)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for CreateHostGroupsDto {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut groups_dto = CreateHostGroupsDto(vec![]);
        let map = HashMap::<String, Vec<RawHost>>::deserialize(d)?;
        for (host_group_name, raw_host) in map {
            let host_dto_list = raw_host
                .into_iter()
                .map(
                    |RawHost {
                         host_name,
                         host_url,
                     }| CurrentHostDto {
                        host_name,
                        host_group_name: host_group_name.clone(),
                        host_url,
                        logs: None,
                    },
                )
                .collect::<Vec<CurrentHostDto>>();
            let group = HostGroupDto::new(host_group_name, host_dto_list);
            groups_dto.0.push(group);
        }
        Ok(groups_dto)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn setup_json() -> &'static str {
        let json = r#"{
            "group1": [
                { "name": "host1", "url": "http://host1.com" },
                { "name": "host2", "url": "http://host2.com" }
            ],
            "group2": [
                { "name": "host3", "url": "http://host3.com" }
            ]
        }"#;
        json
    }
    fn setup_host_groups() -> CreateHostGroupsDto {
        let dto: CreateHostGroupsDto = CreateHostGroupsDto(vec![
            HostGroupDto {
                host_group_name: "group1".to_string(),
                hosts: vec![
                    CurrentHostDto {
                        host_name: "host1".to_string(),
                        host_group_name: "group1".to_string(),
                        host_url: "http://host1.com".to_string(),
                        logs: None,
                    },
                    CurrentHostDto {
                        host_name: "host2".to_string(),
                        host_group_name: "group1".to_string(),
                        host_url: "http://host2.com".to_string(),
                        logs: None,
                    },
                ],
            },
            HostGroupDto {
                host_group_name: "group2".to_string(),
                hosts: vec![CurrentHostDto {
                    host_name: "host3".to_string(),
                    host_group_name: "group2".to_string(),
                    host_url: "http://host3.com".to_string(),
                    logs: None,
                }],
            },
        ]);
        dto
    }

    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace() && *c != '\n').collect()
    }

    #[test]
    fn test_json_serialization() {
        // remove whitespace from both strings before comparison
        let json = setup_json();
        let dto = setup_host_groups();
        let serialized = serde_json::to_string_pretty(&dto).unwrap();
        dbg!(&remove_whitespace(&serialized));
        assert_eq!(
            remove_whitespace(&serialized),
            remove_whitespace(json)
        );
    }
    #[test]
    fn test_json_deserialization() {
        let json = setup_json();
        let dto = setup_host_groups();
        let deserialized: CreateHostGroupsDto = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, dto);
    }
}
