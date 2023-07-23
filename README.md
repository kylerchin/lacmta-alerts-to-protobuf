# Los Angeles Protobuf Alerts

Los Angeles Metro currently does not share alerts with the public in the Protobuf format. However, the service alerts seems slightly compatible. It is presented in Json format.
I was able to convert it to Protobuf using the `gtfs_rt` crate.

My version of the feeds are hosted at 

`https://kactusapi.kylerchin.com/gtfsrtasjson/?feed=f-metro~losangeles~bus~rt&category=alerts`

and

`https://kactusapi.kylerchin.com/gtfsrtasjson/?feed=f-metro~losangeles~rail~rt&category=alerts`
