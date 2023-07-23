use color_eyre::eyre::Result;
use redis::Commands;
use redis::RedisError;
use redis::{Client as RedisClient, RedisResult};

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    /*
    let result = lacmta_alerts_protobuf::req_into_split_feeds_bytes()
        .await
        .unwrap();

    println!("Bus {:?} bytes", result.bus.len());
    println!("Rail {:?} bytes", result.rail.len());



     let result = lacmta_alerts_protobuf::req_into_split_feeds()
        .await
        .unwrap();

    println!("Bus {:#?} ", result.bus);
    println!("Rail {:#?} ", result.rail);

    */
    let redisclient = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let mut con = redisclient.get_connection().unwrap();

    let mut lastloop = Instant::now();

    let bus_id = "f-metrolosangeles~bus~rt";

    let rail_id = "f-metrolosangeles~rail~rt";

    loop {
        lastloop = Instant::now();

        println!("Downloading");

        let result = lacmta_alerts_protobuf::req_into_split_feeds_bytes().await;

        println!("Got it, ingesting");

        match result {
            Ok(result) => {
                let _: () = con
                    .set(format!("gtfsrt|{}|alerts", bus_id), result.bus)
                    .unwrap();

                let _: () = con
                    .set(format!("gtfsrt|{}|alerts", rail_id), result.rail)
                    .unwrap();

                let _: () = con
                    .set(
                        format!("gtfsrttime|{}|alerts", rail_id),
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                            .to_string(),
                    )
                    .unwrap();

                let _: () = con
                    .set(
                        format!("gtfsrttime|{}|alerts", bus_id),
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                            .to_string(),
                    )
                    .unwrap();

                let _: () = con
                    .set(
                        format!("gtfsrtexists|{}", &bus_id),
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                            .to_string(),
                    )
                    .unwrap();

                let _: () = con
                    .set(
                        format!("gtfsrtexists|{}", &rail_id),
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis()
                            .to_string(),
                    )
                    .unwrap();
            }
            Err(error) => {
                println!("{:#?}", error)
            }
        }

        let duration = lastloop.elapsed();

        //if the iteration of the loop took <10 seconds, sleep for the remainder of the time
        if (duration.as_millis() as i32) < 10_000 {
            let sleep_duration = Duration::from_millis(10000) - duration;
            println!("sleeping for {:?}", sleep_duration);
            std::thread::sleep(sleep_duration);
        }
    }
}
