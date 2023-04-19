## Testing the API with grpcurl

See what services we have deployed:

`grpcurl -plaintext localhost:7403 list`

List methods on a service

`grpcurl -plaintext localhost:7403 list trace.Trace`

Call a method

`grpcurl -H 'x-api-key: API_KEY' -plaintext localhost:7403 trace.Trace/processNegotiations`

Call a method via the Rest API, this needs to go via envoy.

`curl -H 'x-api-key: API_KEY' -X POST localhost:7400/trace.Trace/processNegotiations`

Post XML file

`export XML_FILE=$(cat OEM.xml)`

`curl -H 'x-api-key: YOWMcHIelrhUcNdzfc97R606DaZFwA' -i -X POST localhost:7400/trace.Trace/UploadXMLData -d "{'msg': '$XML_FILE'}"`