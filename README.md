# Los Angeles Protobuf Alerts

Los Angeles Metro currently does not share alerts with the public in the Protobuf format. However, the service alerts page json seems compatible. It is presented in Json format.
I was able to convert it to Protobuf using the `gtfs_rt` crate.

My version of the feeds are hosted at 

`https://kactusapi.kylerchin.com/gtfsrtasjson/?feed=f-metro~losangeles~bus~rt&category=alerts`

and

`https://kactusapi.kylerchin.com/gtfsrtasjson/?feed=f-metro~losangeles~rail~rt&category=alerts`


### Running the binary

The runtime `bin/main.rs` is a script that runs `req_into_split_feeds_bytes()` every 10 seconds and inserts the compressed protobuf bytes into redis keys `gtfsrt|f-metrolosangeles~rail~rt|alerts` and `gtfsrt|f-metrolosangeles~rail~rt|alerts`

This is compatible with the [kylerchin/kactus-gtfs-rt](https://github.com/kylerchin/kactus-gtfs-rt/) server. It can be published as a web API using the kactus server.

### using the raw functions in your own project

if you want to integrate this into your own project and not use the redis cache, here are the functions in the library.

`download_to_structure()` puts the entire alert into a single `gtfs_rt::FeedMessage`

`req_into_split_feeds()` calls `download_to_structure()` and splits it into a struct 

```rust
pub struct SplitFeeds {
    pub bus: gtfs_rt::FeedMessage,
    pub rail: gtfs_rt::FeedMessage,
}
```
`req_into_split_feeds_bytes()` calls `req_into_split_feeds()` but is the compressed protobuf version
```rust
pub struct split_feeds_bytes {
    pub bus: Vec<u8>,
    pub rail: Vec<u8>,
}
```
