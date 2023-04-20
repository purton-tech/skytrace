+++
title = "Uploading XML"
date = 2023-01-27
weight = 20
template = "docs-page.html"
+++

The Skytrace API supports uploading XML files via the `UploadXMLData` method. This method is part of the `trace` service.

## Setup

First export the file you want to upload into and environment variable:

```bash
export XML_FILE=$(cat path/to/file/FILENAME.xml)
```

You'll need to create an API key in the web appliation.

Then run the following curl command:

```bash
curl -H 'x-api-key: YOU_API_KEY' -i -X POST https://app.skytrace.space/trace.Trace/UploadXMLData -d "{'msg': '$XML_FILE'}"
```

