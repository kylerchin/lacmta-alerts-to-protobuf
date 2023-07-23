use gtfs_rt::EntitySelector;
use gtfs_rt::TimeRange;
use prost::Message;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;
use std::error::Error;
use std::time::{Duration, SystemTime};

#[derive(Debug, Deserialize, Clone)]
struct lametro_json_response {
    alerts: Vec<lametro_json_alert>,
}

#[derive(Debug, Deserialize, Clone)]
struct lametro_effect {
    effect_start: String,
    effect_end: String,
}

#[derive(Debug, Deserialize, Clone)]
struct lametro_services_list {
    services: Vec<lametro_services>,
}

fn string_or_i32_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrI32 {
        Str(String),
        I32(i32),
    }

    Ok(match StrOrI32::deserialize(deserializer)? {
        StrOrI32::Str(v) => v.parse().unwrap_or(0), // Ignoring parsing errors
        StrOrI32::I32(v) => v,
    })
}

#[derive(Debug, Deserialize, Clone)]
struct lametro_services {
    #[serde(deserialize_with = "string_or_i32_to_i32")]
    route_type: i32,
    mode_name: String,
    route_id: String,
    route_name: String,
    stop_id: Option<String>,
    stop_name: Option<String>,
}

fn str_or_u64_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU64 {
        Str(String),
        U64(u64),
    }

    Ok(match StrOrU64::deserialize(deserializer)? {
        StrOrU64::Str(v) => v, // Ignoring parsing errors
        StrOrU64::U64(v) => v.to_string(),
    })
}

#[derive(Debug, Deserialize, Clone)]
struct lametro_json_alert {
    #[serde(deserialize_with = "str_or_u64_to_string")]
    alert_id: String,
    effect_name: String,
    effect: String,
    cause_name: Option<String>,
    cause: String,
    header_text: Option<String>,
    short_header_text: Option<String>,
    description_text: Option<String>,
    severity: String,
    created_dt: String,
    last_modified_dt: String,
    service_effect_text: String,
    timeframe_text: String,
    alert_lifecycle: String,
    effect_periods: Vec<lametro_effect>,
    affected_services: lametro_services_list,
}

fn gtfs_string_to_timestamp(x: String) -> Option<u64> {
    if x.is_empty() {
        return None;
    } else {
        let pre_result = "123".parse::<u64>();

        match pre_result {
            Ok(y) => Some(y),
            Err(y) => None,
        }
    }
}

pub async fn download_to_structure() -> Result<gtfs_rt::FeedMessage, Box<dyn Error>> {
    let body = reqwest::get("https://alerts.metroservices.io/developer/api/v2/alerts?api_key=4oJedLBt80WP-d7E6Ekf5w&format=json")
    .await?
    .json::<lametro_json_response>().await?;

    //println!("body = {:#?}", body);

    let entity: Vec<gtfs_rt::FeedEntity> = body
        .alerts
        .into_iter()
        .map(|alert| {
            let header_translation: gtfs_rt::TranslatedString = gtfs_rt::TranslatedString {
                translation: vec![gtfs_rt::translated_string::Translation {
                    text: alert.header_text.unwrap(),
                    language: Some("en".to_string()),
                }],
            };

            let informed_entity: Vec<EntitySelector> = alert
                .affected_services
                .services
                .into_iter()
                .map(|service| EntitySelector {
                    agency_id: None,
                    route_id: Some(service.route_id),
                    stop_id: service.stop_id,
                    trip: None,
                    route_type: Some(service.route_type),
                })
                .collect();

            let active_period: Vec<TimeRange> = alert
                .effect_periods
                .into_iter()
                .map(|effect_period| TimeRange {
                    start: gtfs_string_to_timestamp(effect_period.effect_start),
                    end: gtfs_string_to_timestamp(effect_period.effect_end),
                })
                .collect();

            let cause:i32 = match alert.cause.as_str() {
                "UNKNOWN_CAUSE" => 1,
                "OTHER_CAUSE" => 2,  // Not machine-representable.
                "TECHNICAL_PROBLEM" => 3,
                "STRIKE" => 4,         // Public transit agency employees stopped working.
                "DEMONSTRATION" => 5,  // People are blocking the streets.
                "ACCIDENT" => 6,
                "HOLIDAY" => 7,
                "WEATHER" => 8,
                "MAINTENANCE" => 9,
                "CONSTRUCTION" => 10,
                "POLICE_ACTIVITY" => 11,
                "MEDICAL_EMERGENCY" => 12,
                _ => 1 //unknown
            };

            let effect:i32 = match alert.effect.as_str() {
                "NO_SERVICE" => 1,
                "REDUCED_SERVICE" => 2,
                "SIGNIFICANT_DELAYS" => 3,
                "DETOUR" => 4,
                "ADDITIONAL_SERVICE" => 5,
                "MODIFIED_SERVICE" => 6,
                "OTHER_EFFECT" => 7,
                "UNKNOWN_EFFECT" => 8,
                "STOP_MOVED" => 9,
                _ => 8 //known
            };

            gtfs_rt::FeedEntity {
                alert: Some(gtfs_rt::Alert {
                    //todo: cause and effect
                    cause: Some(cause),
                    effect: Some(effect),
                    header_text: Some(header_translation),
                    description_text: match alert.description_text {
                        Some(description_text) => Some(gtfs_rt::TranslatedString {
                            translation: vec![gtfs_rt::translated_string::Translation {
                                text: description_text,
                                language: Some("en".to_string()),
                            }],
                        }),
                        None => None,
                    },
                    url: Some(gtfs_rt::TranslatedString {
                        translation: vec![gtfs_rt::translated_string::Translation {
                            text: "https://www.metro.net/service/advisories".to_string(),
                            language: Some("en".to_string()),
                        }],
                    }),
                    informed_entity: informed_entity,
                    active_period: active_period,
                }),
                vehicle: None,
                trip_update: None,
                is_deleted: None,
                id: alert.alert_id,
            }
        })
        .collect();

    return Ok(gtfs_rt::FeedMessage {
        header: gtfs_rt::FeedHeader {
            incrementality: None,
            gtfs_realtime_version: "2.0".to_string(),
            timestamp: Some(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
        },
        entity: entity,
    });
}

/*
pub async fn download_to_protobuf_compressed() -> Result<Vec<u8>, Box<dyn Error>> {

    let download_struct: Result<gtfs_rt::FeedMessage, Box<dyn Error>> = download_to_structure().await;

    match download_struct {
        Ok(result) => {
            Ok(result.encode_to_vec())
        },
        Err(result) => {
            Err(result)
        }
    }
}*/

pub struct SplitFeeds {
    pub bus: gtfs_rt::FeedMessage,
    pub rail: gtfs_rt::FeedMessage,
}

pub enum WhichFeed {
    Bus,
    Rail,
}

fn should_entity_be_allowed(entity: gtfs_rt::FeedEntity, whichfeed: WhichFeed) -> bool {
    if entity.alert.is_some() {
        let alert: gtfs_rt::Alert = entity.alert.unwrap();

        let list_of_route_type: Vec<Option<i32>> = alert
            .informed_entity
            .into_iter()
            .map(|entity| entity.route_type)
            .collect();

        match whichfeed {
            WhichFeed::Bus => list_of_route_type.contains(&Some(3)),
            WhichFeed::Rail => {
                list_of_route_type.contains(&Some(0))
                    || list_of_route_type.contains(&Some(1))
                    || list_of_route_type.contains(&Some(2))
                    || list_of_route_type.contains(&Some(7))
            }
        }
    } else {
        false
    }
}

pub async fn req_into_split_feeds() -> Result<SplitFeeds, Box<dyn Error>> {
    let structure: Result<gtfs_rt::FeedMessage, Box<dyn Error>> = download_to_structure().await;

    match structure {
        Ok(structure) => Ok(SplitFeeds {
            bus: gtfs_rt::FeedMessage {
                header: structure.clone().header,
                entity: structure
                    .clone()
                    .entity
                    .into_iter()
                    .filter(|entity: &gtfs_rt::FeedEntity| {
                        should_entity_be_allowed(entity.clone(), WhichFeed::Bus)
                    })
                    .collect(),
            },
            rail: gtfs_rt::FeedMessage {
                header: structure.clone().header,
                entity: structure
                    .clone()
                    .entity
                    .into_iter()
                    .filter(|entity: &gtfs_rt::FeedEntity| {
                        should_entity_be_allowed(entity.clone(), WhichFeed::Rail)
                    })
                    .collect(),
            },
        }),
        Err(structure) => Err(structure),
    }
}

pub struct split_feeds_bytes {
    pub bus: Vec<u8>,
    pub rail: Vec<u8>,
}

pub async fn req_into_split_feeds_bytes() -> Result<split_feeds_bytes, Box<dyn Error>> {
    let split = req_into_split_feeds().await;

    match split {
        Ok(split) => Ok(split_feeds_bytes {
            bus: split.bus.encode_to_vec(),
            rail: split.rail.encode_to_vec(),
        }),
        Err(split) => Err(split),
    }
}
