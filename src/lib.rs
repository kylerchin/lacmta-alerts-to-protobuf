use std::error::Error;
use std::time::{Duration, SystemTime};
use prost::Message;
use serde::{Serialize, Deserialize};
use gtfs_rt::EntitySelector;
use gtfs_rt::TimeRange;

#[derive(Debug, Deserialize, Clone)]
struct lametro_json_response {
    alerts: Vec<lametro_json_alert>
}

#[derive(Debug, Deserialize, Clone)]
struct lametro_effect {
    effect_start: String,
    effect_end: String
}

#[derive(Debug, Deserialize, Clone)]
struct lametro_services_list {
    services: Vec<lametro_services>
}

#[derive(Debug, Deserialize, Clone)]
struct lametro_services {
    route_type: i32,
    mode_name: String,
    route_id: String,
    route_name: String,
    stop_id: Option<String>,
    stop_name: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
struct lametro_json_alert {
    alert_id: u32,
    effect_name: String,
    effect: String,
    cause_name: String,
    cause: String,
    header_text: String,
    short_header_text: String,
    description_text: String,
    severity: String,
    created_dt: String,
    last_modified_dt: String,
    service_effect_text: String,
    timeframe_text: String,
    alert_lifecycle: String,
    effect_periods: Vec<lametro_effect>,
    affected_services: lametro_services_list
}

fn gtfs_string_to_timestamp(x: String) -> Option<u64> {
    if x.is_empty() {
        return None;
    } else {
        let pre_result = "123".parse::<u64>();

        match pre_result {
            Ok(y) => {
                Some(y)
            },
            Err(y) => {
                None
            }
        }
    }
}

pub async fn download_to_structure() -> Result<gtfs_rt::FeedMessage, Box<dyn Error>> 
 {
    let body = reqwest::get("https://alerts.metroservices.io/developer/api/v2/alerts?api_key=4oJedLBt80WP-d7E6Ekf5w&format=json")
    .await?
    .json::<lametro_json_response>().await?;

    println!("body = {:#?}", body);

  
            let entity: Vec<gtfs_rt::FeedEntity> = body.alerts.into_iter().map(
                |alert| {

                    let header_translation: gtfs_rt::TranslatedString = gtfs_rt::TranslatedString {
                        translation: vec![
                            gtfs_rt::translated_string::Translation {
                                text: alert.header_text,
                                    language: Some("en".to_string())
                            }
                        ]
                    };

                    let informed_entity: Vec<EntitySelector> = alert.affected_services.services.into_iter()
                    .map(|service| {
                        EntitySelector {
                            agency_id: None,
                            route_id: Some(service.route_id),
                            stop_id: service.stop_id,
                            trip: None,
                            route_type: Some(service.route_type)
                        }
                    }).collect();

                    let active_period: Vec<TimeRange> = alert.effect_periods.into_iter().map(|effect_period| {
                        TimeRange {
                            start: gtfs_string_to_timestamp(effect_period.effect_start),
                            end: gtfs_string_to_timestamp(effect_period.effect_end)
                        }
                    }).collect();

                    gtfs_rt::FeedEntity {
                        alert: Some(gtfs_rt::Alert {
                            cause: Some(0),
                            effect: Some(0),
                            header_text: Some(header_translation),
                            description_text: Some(gtfs_rt::TranslatedString {
                                translation: vec![
                                    gtfs_rt::translated_string::Translation {
                                        text: alert.description_text,
                                            language: Some("en".to_string())
                                    }
                                ]
                            }),
                            url: Some(gtfs_rt::TranslatedString {
                                translation: vec![
                                    gtfs_rt::translated_string::Translation {
                                        text: "https://www.metro.net/service/advisories".to_string(),
                                        language: Some("en".to_string())
                                    }
                                ]
                            }),
                            informed_entity: informed_entity,
                            active_period: active_period
                        }),
                        vehicle: None,
                        trip_update: None,
                        is_deleted: None,
                        id: alert.alert_id.to_string()
                    }
                }
            ).collect();

            
    Ok(gtfs_rt::FeedMessage {
        header: gtfs_rt::FeedHeader {
            incrementality: None,
            gtfs_realtime_version: "2.0".to_string(),
            timestamp: Some(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
        },
        entity: entity
    })
        
    

   

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

struct SplitFeeds {
    bus: gtfs_rt::FeedMessage,
    rail: gtfs_rt::FeedMessage
}

pub async fn req_into_split_feeds() -> Result<SplitFeeds, Box<dyn Error>> {
    let structure = download_to_structure().await;

    match structure {
        Ok(structure) => {
            split_feeds {
                bus: structure.,
            }
        },
        Err(structure) => {
            Err(structure)
        }
    }
}

struct split_feeds_bytes {
    
}

pub async fn req_into_split_feeds_bytes() -> Result<>

#[cfg(test)]
mod tests {
    use super::*;

    async fn test() {
        let download = download_to_structure().await;

        assert!(download.is_ok())
    }
}
