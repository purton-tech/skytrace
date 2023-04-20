## Testing the API with grpcurl

See what services we have deployed:

`grpcurl -plaintext localhost:7403 list`

List methods on a service

`grpcurl -plaintext localhost:7403 list trace.Trace`

Call a method

`grpcurl -H 'x-api-key: API_KEY' -plaintext localhost:7403 trace.Trace/processNegotiations`

Call a method via the Rest API, this needs to go via envoy.

`curl -H 'x-api-key: API_KEY' -i -X POST localhost:7400/trace.Trace/processNegotiations`

Post an XML file

`export XML_FILE=$(cat crates/grpc-api/testdata/OEM.xml)`

`curl -H 'x-api-key: YOWMcHIelrhUcNdzfc97R606DaZFwA' -i -X POST envoy:7400/trace.Trace/UploadXMLData -d "{'msg': '$XML_FILE'}"`

## Testing via cloudflared

`docker run --network skytrace_devcontainer_default cloudflare/cloudflared tunnel --loglevel debug --url envoy:7400`

## Test against Production

Use `https://app.skytrace.space/trace.Trace/UploadXMLData` as the host.