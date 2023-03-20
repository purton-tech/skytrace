+++
title = "SkyTrace API"
date = 2023-01-27
weight = 10
template = "docs-page.html"
+++

SkyTrace uses gRPC for our API. gRPC is a modern open source high performance Remote Procedure Call (RPC) framework that can run in any environment. 

This allows you to code generate client stubs or an SDK for whichever programming language you wish to use.

We also expose the API over [gRPC Web](https://github.com/grpc/grpc-web) so you can completely bypass the SDK generate step and use the API using Restful requests with `curl` for example.

## gRPC Schema

The top level schema looks like the following we currently only support CCSDS data upload and download.

```proto
syntax = "proto3";

import "ccsds/schema/cdm.proto";
import "ccsds/schema/oem.proto";
import "ccsds/schema/opm.proto";

package trace;

service Trace {
    // Uploads a single CCSDS data message.
    // Failures are indicated using gRPC error codes.
    //
    // *Note*: This method can be called within a loop for batch data uploads.
    rpc UploadData(UploadDataRequest) returns (EmptyResponse) {};

    // Find any hies and process.
    rpc processNegotiations(EmptyRequest) returns (EmptyResponse) {};
}

// CCSDS data message in protocol buffer format (i.e., CDM, TDM, ...).
message DataMessage {
    oneof data {
      // CCSDS Conjunction Data Message (CDM).
      ccsds.schema.CdmType cdm = 1;
      // CCSDS Orbit Message (OEM).
      ccsds.schema.OemType oem = 2;
      // CCSDS Tracking Data Message (OPM).
      ccsds.schema.OpmType opm = 3;
    }
}

// CCSDS data upload request.
message UploadDataRequest {
    // CCSDS data message to upload.
    DataMessage msg = 1;
    // Signature of the data message encoded in proto3 binary format, using the originators private ECDSA key.
    string signature = 2;
}

message EmptyRequest {
}
  
message EmptyResponse {
}
```