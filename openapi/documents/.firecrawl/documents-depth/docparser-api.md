[NAV\\
 ![](https://docparser.com/api/images/navbar-cad8cdcb.png)](https://docparser.com/api/#)

![](https://docparser.com/api/images/logo-f46ab3cb.png)

[CURL](https://docparser.com/api/#) [PHP](https://docparser.com/api/#) [Python](https://docparser.com/api/#) [Node.js](https://docparser.com/api/#) [JSON Response](https://docparser.com/api/#)

- [Getting Started](https://docparser.com/api/#getting-started)  - [Introduction](https://docparser.com/api/#introduction)
  - [Client Libraries (SDKs)](https://docparser.com/api/#client-libraries-sdks)
- [Authentication](https://docparser.com/api/#authentication)  - [HTTP Basic Auth](https://docparser.com/api/#http-basic-auth)
  - [With API Key](https://docparser.com/api/#with-api-key)
- [Parsers](https://docparser.com/api/#parsers)  - [List Document Parsers](https://docparser.com/api/#list-document-parsers)
  - [List Parser Model Layouts](https://docparser.com/api/#list-parser-model-layouts)
- [Documents](https://docparser.com/api/#documents)  - [Import Documents](https://docparser.com/api/#import-documents)
  - [Document Status](https://docparser.com/api/#document-status)
- [Parsed Data](https://docparser.com/api/#parsed-data)  - [Get Data Of One Document](https://docparser.com/api/#get-data-of-one-document)
  - [Get Data Of Multiple Documents](https://docparser.com/api/#get-data-of-multiple-documents)
  - [Re-Parse Data](https://docparser.com/api/#re-parse-data)
  - [Re-Integrate Data](https://docparser.com/api/#re-integrate-data)

- [Discover Docparser](https://docparser.com/)
- [Visit Docparser App](https://app.docparser.com/)
- [Terms Of Service](https://docparser.com/legal/terms-of-service)

# Getting Started

## Introduction

Welcome to the API of Docparser! You can use this API to

- list Document Parsers created with Docparser
- load documents to a Document Parser
- obtain your parsed data

The Docparser API is organized around REST principles. Our API has predictable, resource-oriented URLs, and uses clear response messages to indicate API errors.

The code examples in the right sidebar are designed to show you how to call our API. All you need to do is to replace the `secret_api_key` in the sample with your [private API token](https://app.docparser.com/myaccount/api).

This documentation was last updated 2018-02-13.

## Client Libraries (SDKs)

Docparser comes with two official client libraries to make it easier for you to build an integration with Docparser.

Both client libraries are open source and are published under the MIT license. Which means that you can use them in your projects without any restrictions. If you want to contribute to the development of our libraries, please don't hesitate to create a pull-requests in our Github repositories.

Please note that the Docparser API can be used with any programming language capable of making HTTPS calls - even if there is no ready-to-use client library available. In case you developed a library in another programming language than the ones listed below, we would be thrilled to list it here as open source.

**Official Libraries**

- [PHP Client Library](https://github.com/Docparser/Docparser-PHP)
- [Node.js Client Library](https://github.com/Docparser/Docparser-Node)

**Third Party Libraries And Code Snippets**

- [Python Client Library](https://pypi.org/project/PyDocParser)
- [Salesforce Apex Code Snippets](https://github.com/Docparser/Docparser-Apex)
- [Coldfusion Library](https://github.com/Construction-Monitor/coldfusion-docparser)

# Authentication

Every request to the Docparser API needs to be authenticated with a secret API key linked to your account. You can obtain and reset your secret API key in the [API Settings](https://app.docparser.com/myaccount/api) of your Docparser Account. Your API key carries many privileges, so be sure to keep them secret!

All API requests must be made over HTTPS. Calls made over plain HTTP will fail. API requests without authentication will also fail.

Authentication to the API can be performed in two ways:

- via HTTP Basic Auth (recommended)
- by directly providing your API key in your request

You can test if the authentication works by pinging the following URL. Please make sure to include the correct authentication headers / parameters as described below.

`GET https://api.docparser.com/v1/ping`

## HTTP Basic Auth

```
curl https://api.docparser.com/v1/ping \
   -u <secret_api_key>:
```

```

```

```

```

```
{"msg": "pong"}
```

This authentication method is the preferred way of authenticating your requests to Docparser.
When using HTTP Basic Auth, use your secret API key as the "username" and leave the "password" blank.

## With API Key

```
curl https://api.docparser.com/v1/ping -H 'api_key: <secret_api_key>'

curl https://api.docparser.com/v1/ping?api_key=<secret_api_key>
```

```
require('./vendor/autoload.php');

use Docparser\Docparser;

$docparser = new Docparser("secret_api_key");

echo $docparser->ping();
```

```
import pydocparser

parser = pydocparser.Parser()

parser.login("secret_api_key")

print(parser.ping())
```

```
var docparser = require('docparser-node');

var client = new docparser.Client("secret_api_key");

client.ping()
  .then(function() {
    console.log('authentication succeeded!')
  })
  .catch(function(err) {
    console.log('authentication failed!')
  });
```

```
{"msg": "pong"}
```

In case Basic Auth is not an option for you, it is also possible to include your secret API key directly within your request. You can provide your API key either as a header (api\_key: ABC123), a post-field (api\_key=ABC123) or an URL query parameter (&api\_key=ABC123).

Please note that including your API as an URL query parameter is the least secure method and we don't recommend doing this. Including API keys in URLs comes with a high risk of accidentally exposing them to others.

# Parsers

## List Document Parsers

```
curl https://api.docparser.com/v1/parsers \
   -u <secret_api_key>:
```

```
    $docparser->getParsers();
```

```
parsers = parser.list_parsers()
```

```
    client.getParsers()
    .then(function (parsers) {
        console.log(parsers)
    })
    .catch(function (err) {
        console.log(err)
    })
```

```
[{\
  "id":"mwekrupomwekrupo",\
  "label":"Acme Inc. Invoice Parser"\
},{\
  "id":"cadqtvgjcadqtvgj",\
  "label":"Acme Corp. Invoice Parser"\
}]
```

This endpoint returns a list of all Document Parsers linked to your account. Each entry contains an `id` and a label. The `id` value an be used in other API routes, e.g. for importing documents to a specific Document Parser or obtaining parsing results.

`GET https://api.docparser.com/v1/parsers`

## List Parser Model Layouts

```
curl https://api.docparser.com/v1/parser/models/<PARSER_ID> \
   -u <secret_api_key>:
```

```
[{\
  "id":"1",\
  "label":"Acme Inc. Invoice Parser Layout #1"\
},{\
  "id":"2",\
  "label":"Acme Corp. Invoice Parser Layout #2"\
}]
```

This endpoint returns a list of all the Model Layouts for a specific parser linked to your account.

`GET https://api.docparser.com/v1/parser/models/<PARSER_ID>`

# Documents

## Import Documents

We offer several options to import a document to Docparser to make it as easy as possible for you to integrate Docparser in your existing workflow.

Next to manually uploading your documents with our app, Docparser also allows you to import documents using this API. You can upload your document with a HTTP POST request, or by providing a publicly accessible URL which can be used to fetch the document.

Hint: Another easy way of importing your documents is to forward them by e-mail to a private email inbox linked to your account. You can learn more about this method in the settings of your Document Parser.

### Upload Document From Local Path

```
curl \
  -u <secret_api_key>: \
  -F "file=@/home/your/local/file.jpdf" \
  https://api.docparser.com/v1/document/upload/<PARSER_ID>
```

```
$docparser->uploadDocumentByPath($parserId, $filePath, $remoteId = null);
```

```
document_id = parser.upload_file_by_path("path to document.pdf", "parser name")
```

```
client.uploadFileByPath('PARSER_ID', './test.pdf', {remote_id: 'test'})
  .then(function (result) {
    console.log(result)
  })
  .catch(function (err) {
    console.log(err)
  })
```

```
{
    "id" : "abc123efg456",
    "quota_used" : 642,
    "quota_left" : 258,
    "quota_refill" : "2017-05-02T02:43:48+00:00"
}
```

Docparser allows you to upload documents from your local hard-drive with a multipart/form-data request. This is the same type of request a HTML form with a file upload field would send. The field name used for the document upload needs to be `file`.

The return value of a successful upload is the ID of the newly created document, as well as account usage data.

Each of your Document Parsers has a unique API route to which you need to send your request. The `<PARSER_ID>` shown in the URL below can be obtained by calling the `List Parsers` API route. You can also easily obtain the `<PARSER_ID>` inside the Docparser app in the settings of your Document Parser under `Settings > API`.

In addition, you can submit an arbitrary string to Docparser which will be stored together with the uploaded document. The submitted value (`remote_id`) will be kept throughout the processing and will be available later once you obtain the parsed data with our API, as CSV/XLS/XML file or through webhooks. This optional parameter makes it easy to relate the parsed data returned by Docparser with document records in your own system.

`POST https://api.docparser.com/v1/document/upload/<PARSER_ID>`

| Parameter | Description |
| --- | --- |
| file | The file object to upload |
| remote\_id | Optional parameter to pass through your own document ID |

### Upload Document By Content

```
curl \
  -u <secret_api_key>: \
  -F "file_content=....&file_name=...." \
  https://api.docparser.com/v1/document/upload/<PARSER_ID>
```

```
$docparser->uploadDocumentByContents($parserId, $file, $remoteId = null, $filename = null);
```

```
document_id = parser.upload_file_by_base64(base64_content, "file name.pdf", "parser name")
```

```
client.uploadFileByStream('someparserid', fs.createReadStream('filepath'), options)
  .then(function (result) {
    console.log(result)
  })
  .catch(function (err) {
    console.log(err)
  })
```

```
{
    "id" : "abc123efg456",
    "file_size" : 119540,
    "quota_used" : 642,
    "quota_left" : 258,
    "quota_refill" : "2017-05-02T02:43:48+00:00"
}
```

Alternatively to uploading a document from your hard drive, you can also send files in using a simple form-data HTTP POST request. To make this work, name your form field `file_content` and use base64 encoding for safe delivery of the data. The document name can be transferred in a second form field called `file_name`.

The return value of a successful upload is the ID of the newly created document, the filesize of the imported document as well as account usage data.

Each of your Document Parsers has a unique API route to which you need to send your request. The `<PARSER_ID>` shown in the URL below can be obtained by calling the `List Parsers` API route. You can also easily obtain the `<PARSER_ID>` inside the Docparser app in the settings of your Document Parser under `Settings > API`.

In addition, you can submit an arbitrary string to Docparser which will be stored together with the uploaded document. The submitted value (`remote_id`) will be kept throughout the processing and will be available later once you obtain the parsed data with our API, as CSV/XLS/XML file or through webhooks. This optional parameter makes it easy to relate the parsed data returned by Docparser with document records in your own system.

`POST https://api.docparser.com/v1/document/upload/<PARSER_ID>`

| Parameter | Description |
| --- | --- |
| file\_content | The file content encoded with base64. |
| file\_name | The file name for this document. This parameter is optional and we will attribute a file-name based on the time of uploading if empty. |
| remote\_id | Optional parameter to pass through your own document ID |

### Fetch Document From URL

```
curl \
  -u <secret_api_key>: \
  -F "url=http://www.pdf995.com/samples/pdf.pdf" \
  https://api.docparser.com/v2/document/fetch/<PARSER_ID>
```

```
$docparser->fetchDocumentFromURL($parserId, $url, $remoteId = null);
```

```
document_id = parser.upload_file_by_url(url_of_file, "parser name")
```

```
client.fetchDocumentFromURL('PARSER_ID', 'http://example.com/test.pdf', {remote_id: 'test'})
  .then(function (result) {
    console.log(result)
  })
  .catch(function (err) {
    console.log(err)
  })
```

```
{
    "document_id": "b07b080b357334510e10f5b41567000c",
    "parser_id": "lgaxqwtznuoa",
    "remote_id": "",
    "message": "The document is scheduled to be fetched from the URL you provided and will be processed in a few minutes. You can check the status of the document at https://api.docparser.com/v2/document/status/lgaxqwtznuoa/b07b080b357334510e10f5b41567000c"
}
```

If your files are stored under a publicly accessible URL, you can also import a document by providing the URL to our API. This method is really straight forward and you just need to perform a simple POST or GET request with `url` as the parameter.

Each of your Document Parsers has a unique API route to which you need to send your request. The `<PARSER_ID>` shown in the URL below can be obtained by calling the `List Parsers` API route. You can also easily obtain the `<PARSER_ID>` inside the Docparser app in the settings of your Document Parser under `Settings > API`.

In addition, you can submit an arbitrary string to Docparser which will be stored together with the fetched document. The submitted value (`remote_id`) will be kept throughout the processing and will be available later once you obtain the parsed data with our API, as CSV/XLS/XML file or through webhooks. This optional parameter makes it easy to relate the parsed data returned by Docparser with document records in your own system.

`POST https://api.docparser.com/v2/document/fetch/<PARSER_ID>`

| Parameter | Description |
| --- | --- |
| url | The location of a publicly accessible document |
| remote\_id | Optional parameter to pass through your own document ID |

### Response

| Field | Description |
| --- | --- |
| document\_id | The unique ID of the fetched document. |
| parser\_id | the parser id |
| remote\_id | the remote id that is passed in the request parameters |
| message | This message contains the status URL of the document. You can check the status if the document is imported or not by visiting this URL. |

## Document Status

```
curl \
  -u <secret_api_key>: \
  https://api.docparser.com/v2/document/status/<PARSER_ID>/<DOCUMENT_ID>
```

```

```

```

```

```

```

```
{
    "token": "fa36ba4b7ac507fe76f9388a54c18114",
    "remote_id": "",
    "file_source": "api",
    "filename": "example.name",
    "mime_type": "",
    "pages": 0,
    "supported": true,
    "importing_in_progress": false,
    "processing_in_progress": false,
    "webhook_dispatching_in_progress": false,
    "uploaded_at": 1724028973,
    "imported_at": 0,
    "ocr_started_at": 0,
    "preprocessed_at": 0,
    "preprocessing_in_progress_at": 0,
    "processed_at": 0,
    "first_processed_at": 0,
    "dispatched_webhook": false,
    "dispatched_webhook_at": 0,
    "dispatched_webhook_problem": false,
    "webhooks_created": 0,
    "webhooks_sent": 0,
    "failed_jobs": [\
        "file_fetch_api"\
    ]
}
```

To check the status of a document, this endpoint provides all the information about the document's state, including timestamps and flags. If any job associated with the document fails, it will be listed under the `failed_jobs` field.

`GET https://api.docparser.com/v2/document/status/<PARSER_ID>/<DOCUMENT_ID>`

# Parsed Data

Docparser provides a couple of different ways to obtain the data parsed from your documents. Basically, you have the following three options:

- Create permanent [download links](https://help.docparser.com/hc/en-us/sections/15895324046484-Download-Parsed-Data)
- Send parsed data to your API with [webhooks](https://support.docparser.com/article/1252-what-are-webhooks-and-cloud-integrations)
- Fetch parsed data with this API

## Get Data Of One Document

```
curl \
  -u <secret_api_key>: \
  https://api.docparser.com/v1/results/<PARSER_ID>/<DOCUMENT_ID>
```

```
$docparser->getResultsByDocument($parserId, $documentId, $format = 'object');
```

```
data = parser.get_one_result("parser name", document_id)
```

```
client.getResultsByDocument(parserId, documentId, {format: 'object'})
  .then(function (result) {
    console.log(result)
  })
  .catch(function (err) {
    console.log(err)
  })
```

```
[\
    {\
        "id": "967bcf5658d73c80563072373d5002e3",\
        "document_id": "1d35639d4b53b59e77f737c93cd1d3d7",\
        "remote_id": "your_optional_id",\
        "file_name": "pdf.pdf",\
        "media_link": "https://api.docparser.com/v1/document/media/...",\
        "media_link_original": "https://api.docparser.com/v1/document/media/.../original",\
        "media_link_data": "https://api.docparser.com/v1/document/media/.../data",\
        "page_count": 4,\
        "uploaded_at": "2016-07-27T14:57:05+00:00",\
        "processed_at": "2016-07-27T14:57:10+00:00",\
        "purchase_number": "ABC123",\
        "customer": {\
            "last_name" : "Doe",\
            "first_name" : "John"\
        },\
        "table_data": [{\
            "key" : "value"\
        }, {\
            "key" : "value"\
        },\
        ...\
        ],\
        "....": "...."\
    }\
]
```

This API route returns the parsed data of one document. The response structure is identical to the `list` route above, only that the contains a single object representing the data of the requested document.

A much better way than polling our API for parsed data is to use our \`Webhook\` integration. By using webhooks, parsed data will be pushed to your API immediately after parsing. You'll find the webhook integration in your Document Parser under 'Integrations > Advanced Webhook'.

The `<PARSER_ID>` shown in the URL below can be obtained by calling the `List Parsers` API route. You can also easily obtain the `<PARSER_ID>` inside the Docparser app in the settings of your Document Parser under `Settings > API`.

The `<DOCUMENT_ID>` is returned when uploading/importing a new document.

`GET https://api.docparser.com/v1/results/<PARSER_ID>/<DOCUMENT_ID>`

The results API is subject to a rate limit of **60 calls per minute** for optimal performance. If your usage exceeds this limit, kindly reach out to our Customer Happiness Team for assistance. They will be happy to explore options on the application and provide the support you need.

| Parameter | Default | Description |
| --- | --- | --- |
| format | object | Valid values are `object` or `flat`. By default, parsed document data is returned as nested JSON objects. Setting this parameter to flat will return a simplified version of the parsed data which does not contain flat key/value pairs instead of nested objects. |
| include\_children |  | If child documents were created during preprocessing (e.g. when splitting documents), setting this parameter to `true` ensures that the parsed data of all child documents is returned. |

## Get Data Of Multiple Documents

```
curl \
  -u <secret_api_key>: \
  https://api.docparser.com/v1/results/<PARSER_ID>

curl -G \
  -u <secret_api_key>: \
  https://api.docparser.com/v1/results/<PARSER_ID> \
  -d "sort_by=<SORT_BY>" \
  -d "sort_order=<DESC | ASC>"
```

```
$docparser->getResultsByParser($parserId, $options = []);
```

```
data = parser.get_multiple_results("parser name")
```

```
client.getResultsByParser(parserId, {format: 'object'})
  .then(function (result) {
    console.log(result)
  })
  .catch(function (err) {
    console.log(err)
  })
```

```
[\
    {\
        "id": "967bcf5658d73c80563072373d5002e3",\
        "document_id": "1d35639d4b53b59e77f737c93cd1d3d7",\
        "remote_id": "your_optional_id",\
        "file_name": "pdf.pdf",\
        "media_link": "https://api.docparser.com/v1/document/media/...",\
        "media_link_original": "https://api.docparser.com/v1/document/media/.../original",\
        "media_link_data": "https://api.docparser.com/v1/document/media/.../data",\
        "page_count": 1,\
        "uploaded_at": "2016-07-27T14:57:05+00:00",\
        "processed_at": "2016-07-27T14:57:10+00:00",\
        "purchase_number": "ABC123",\
        "customer": {\
            "last_name" : "Doe",\
            "first_name" : "John"\
        },\
        "table_data": [{\
            "key" : "value"\
        }, {\
            "key" : "value"\
        },\
        ...\
        ],\
        "....": "...."\
    },\
    {\
       ....\
    }\
]
```

This API route returns a list of JSON objects representing the parsed data. By default, the data of the last 100 documents in reverse chronological order. Additional parameters can be used to change this default behaviour.

A much better way than polling our API for parsed data is to use our \`Webhook\` integration. By using webhooks, parsed data will be pushed to your API immediately after parsing. You'll find the webhook integration in your Document Parser under 'Integrations > Advanced Webhook'.

The `<PARSER_ID>` shown in the URL below can be obtained by calling the `List Parsers` API route. You can also easily obtain the `<PARSER_ID>` inside the Docparser app in the settings of your Document Parser under `Settings > API`.

`GET https://api.docparser.com/v1/results/<PARSER_ID>`

The results API is subject to a rate limit of **30 calls per minute** for optimal performance. If your usage exceeds this limit, kindly reach out to our Customer Happiness Team for assistance. They will be happy to explore options on the application and provide the support you need.

| Parameter | Default | Description |
| --- | --- | --- |
| format | object | Valid values are `object` or `flat`. By default, parsed document data is returned as nested JSON objects. Setting this parameter to `flat` will return a simplified version of the parsed data which contains flat key/value pairs instead of nested objects. |
| list | last\_uploaded | Valid values are `last_uploaded`, `uploaded_after` and `processed_after`. By default, the data of the last uploaded documents in reverse chronological order is returned. If set to `uploaded_after`, documents imported after a certain date are returned (see below). If set to `processed_after`, documents parsed after a certain date are returned (see below). |
| limit | 100 | This parameter indicates how many documents to include when the parameter `list` is set to `last_uploaded`. The maximum quantity of documents which can be returned is limited 10,000. |
| date |  | This parameter is mandatory if the parameter `list` is set to `uploaded_after` or `processed_after`. The parameter needs to be a valid ISO 8601 (e.g. 2017-02-12T15:19:21+00:00) date string or a Linux timestamp and determines which documents are included in the return. Please note that the maximum quantity of documents which can be returned is limited 10,000. |
| remote\_id |  | When this parameter is set, only documents having the provided `remote_id` will be returned. The `remote_id` of a document can be set when importing the file via the API ( [see above](https://docparser.com/api/#import-documents)). |
| include\_processing\_queue |  | By default, only documents which are fully processed (imported, preprocessed, parsed) are included in the results. By setting `include_processing_queue` to `true`, files which are not yet entirely processed are included in the results. |
| sort\_by |  | By default, it will be sorted by files as they are uploaded into the system. Valid values are `parsed_at`, `processed_at`, `uploaded_at`, `first_processed_at`, `imported_at`, `integrated_at`, `dispatched_webhook_at`, and `preprocessed_at`. Results will be sorted according to the given value. |
| sort\_order | DESC | Valid values are `ASC` and `DESC`. The results will be sorted in ascending or descending order accordingly. |

## Re-Parse Data

```

curl -X POST \
  -u <secret_api_key>: \
  https://api.docparser.com/v1/document/reparse/<PARSER_ID> \
  -d "document_ids[]=<DOCUMENT_ID_1>" \
  -d "document_ids[]=<DOCUMENT_ID_2>" \
  -d "document_ids[]=<DOCUMENT_ID_3>" \
```

```

```

```

```

```

```

```
{
    "total_reparsed": 3,
    "msg": ""
}
```

This API route will schedule documents for re-parsing.

The `<PARSER_ID>` shown in the URL below can be obtained by calling the `List Parsers` API route. You can also easily obtain the `<PARSER_ID>` inside the Docparser app in the settings of your Document Parser under `Settings > API`.

`POST https://api.docparser.com/v1/document/reparse/<PARSER_ID>`

| Parameter | Default | Description |
| --- | --- | --- |
| document\_ids |  | Valid value is a non-empty array of document ids. These document IDs can be obtained from [Parsed Data API](https://docparser.com/api/?json#get-data-of-multiple-documents) results. The field in the result is `document_id`. |

## Re-Integrate Data

```

curl -X POST \
  -u <secret_api_key>: \
  https://api.docparser.com/v1/document/reintegrate/<PARSER_ID> \
  -d "document_ids[]=<DOCUMENT_ID_1>" \
  -d "document_ids[]=<DOCUMENT_ID_2>" \
  -d "document_ids[]=<DOCUMENT_ID_3>" \
```

```

```

```

```

```

```

```
{
    "total_reintegrate": 3,
    "msg": ""
}
```

This API route will schedule documents for integration queue.

The `<PARSER_ID>` shown in the URL below can be obtained by calling the `List Parsers` API route. You can also easily obtain the `<PARSER_ID>` inside the Docparser app in the settings of your Document Parser under `Settings > API`.

`POST https://api.docparser.com/v1/document/reintegrate/<PARSER_ID>`

| Parameter | Default | Description |
| --- | --- | --- |
| document\_ids |  | Valid value is a non-empty array of document ids. These document IDs can be obtained from [Parsed Data API](https://docparser.com/api/?json#get-data-of-multiple-documents) results. The field in the result is `document_id`. |

[CURL](https://docparser.com/api/#) [PHP](https://docparser.com/api/#) [Python](https://docparser.com/api/#) [Node.js](https://docparser.com/api/#) [JSON Response](https://docparser.com/api/#)