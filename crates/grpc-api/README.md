## Testing the API with grpcurl

See what services we have deployed:

`grpcurl -plaintext localhost:7403 list`

List methods on a service

`grpcurl -plaintext localhost:7403 list trace.Trace`

Call a method

`grpcurl -plaintext localhost:7403 trace.Trace/processNegotiations`