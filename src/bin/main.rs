use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let result = lacmta_alerts_protobuf::req_into_split_feeds_bytes()
        .await
        .unwrap();

    println!("Bus {:?} bytes", result.bus.len());
    println!("Rail {:?} bytes", result.rail.len());

    Ok(())
}
