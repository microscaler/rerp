[Jump to Content](https://docs.docupipe.ai/reference/getting-started-with-docupipe#content)

[![DocuPipe](https://files.readme.io/01cb1d38ade80c2365d1f1f5697beffbd859a33682e694a4d4994e53d648aae9-Logo_with_shadow.png)](https://docs.docupipe.ai/docs)

[Guides](https://docs.docupipe.ai/docs) [API Reference](https://docs.docupipe.ai/reference)

* * *

[Log In](https://docs.docupipe.ai/login?redirect_uri=/reference/getting-started-with-docupipe) [![DocuPipe](https://files.readme.io/01cb1d38ade80c2365d1f1f5697beffbd859a33682e694a4d4994e53d648aae9-Logo_with_shadow.png)](https://docs.docupipe.ai/docs)

API Reference

[Log In](https://docs.docupipe.ai/login?redirect_uri=/reference/getting-started-with-docupipe)

[Guides](https://docs.docupipe.ai/docs) [API Reference](https://docs.docupipe.ai/reference)

Getting Started With DocuPipe API

Search
`CTRL-K`

# Getting Started With DocuPipe API

Copy Page

Take your first steps by uploading a document and getting its parsed text and tables.

The heart of DocuPipe is its ability to convert any document to a standard output that has consistently defined fields. We call this extraction process **standardization**.

To make standardization useful, define a consistent structure and clarify how you want DocuPipe to interpret each document type. We call this set of definitions a **schema**. Think of a schema as a collection of slots for information you expect to find (e.g., a rental lease might need _monthly amount_ and _lease end date_ values).

DocuPipe makes it easy to describe how you want to understand your documents with minimal effort.

The workflow consists of three parts:

1. **Upload** a document.
2. **Define** what you want to extract from this document type. This generates a **Schema**.
3. **Extract** results from many documents, using your schema. This generates a **Standardization**.

## Upload Documents   [Skip link to Upload Documents](https://docs.docupipe.ai/reference/getting-started-with-docupipe\#upload-documents)

This tutorial will follow along a toy problem of standardizing rental leases. You can follow along with our example, or extract a completely different document.

Here's an [example lease](https://www.docupipe.ai/demo-docs/leases/lease_trimmed.pdf), which you can use to follow along this guide.

First order of business is to upload a document. You can do this manually from the [Documents dashboard](https://www.docupipe.ai/dashboard/documents/overview) or programmatically with the API.

### Posting a Document with Code   [Skip link to Posting a Document with Code](https://docs.docupipe.ai/reference/getting-started-with-docupipe\#posting-a-document-with-code)

First order of business it to use our API to [Submit a Document for Processing](https://docs.docupipe.ai/reference/post_document-1). Replace `YOUR_API_KEY` with your actual API key obtained in the previous step. Supported file formats include PDF, images (JPG, PNG, WEBP), text files, and JSON. Regardless of the format, always base-64 encode your input document as shown below.

PythonJavaScript

```python
import base64
import requests

url = "https://app.docupipe.ai/document"
api_key = "YOUR_API_KEY"

payload = {"document": {"file": {
    "contents": base64.b64encode(open("example_document.pdf", 'rb').read()).decode(),
    "filename": "example_document.pdf"
}}}
headers = {
    "accept": "application/json",
    "content-type": "application/json",
    "X-API-Key": api_key
}

response = requests.post(url, json=payload, headers=headers)
document_id = response.json()['documentId']
```

```javascript
const fetch = require('node-fetch');
const fs = require('fs');

// Replace with your actual DocuPipe API key
const api_key = "YOUR_API_KEY";
const url = "https://app.docupipe.ai/document";

// Read and encode the file in base64
const filePath = "example_document.pdf";
const fileContents = fs.readFileSync(filePath);
const base64Content = Buffer.from(fileContents).toString('base64');

// Construct the JSON payload
const payload = {
    document: {
        file: {
            contents: base64Content,
            filename: filePath
        }
    }
};

// Make the POST request with JSON payload
fetch(url, {
    method: 'POST',
    headers: {
        "Accept": "application/json",
        "Content-Type": "application/json",
        "X-API-Key": api_key
    },
    body: JSON.stringify(payload)
})
.then(response => response.json())
.then(data => {
    const document_id = data.documentId;
    console.log(document_id); // Output the document ID
})
.catch(error => console.error('Error:', error));
```

If you print the response, you'll see it returns the document ID and the job ID. You can use those identifiers later to fetch AI- and human-readable results:

JSON

```json
print(response)
=> {'documentId': '96dde1aa', 'jobId': '42ace16a'}
```

That response is essentially a **pointer** you can use to query the document's results with a `GET` request.

### Polling for Upload Job Completion   [Skip link to Polling for Upload Job Completion](https://docs.docupipe.ai/reference/getting-started-with-docupipe\#polling-for-upload-job-completion)

As soon as you upload a document, DocuPipe extracts the underlying text, tables, and a clean textual representation for both human and AI readers. This can take seconds or minutes, depending on the file size.

Poll the job endpoint (or listen for a webhook) until processing completes:

PythonJavaScript

```python
import time
import requests

job_id = "42ace16a"

def poll_job(job_id):
    url = f"https://app.docupipe.ai/job/{job_id}"
    headers = {
        "accept": "application/json",
        "X-API-Key": "YOUR_API_KEY"
    }

    status = "processing"
    wait_seconds = 2
    total_attempts = 0

    while status == "processing":
        total_attempts += 1
        if total_attempts > 10:
            raise RuntimeError("failed to parse document")

        response = requests.get(url, headers=headers)
        response.raise_for_status()  # good practice
        status = response.json().get("status")
        print(status)

        time.sleep(wait_seconds)
        wait_seconds *= 2  # exponential backoff

    return response.json()

print(poll_job(job_id))
```

```javascript
import time
import requests

job_id = "42ace16a"

def poll_job(job_id):
    url = f"https://app.docupipe.ai/job/{job_id}"
    headers = {
        "accept": "application/json",
        "X-API-Key": "YOUR_API_KEY"
    }

    status = "processing"
    wait_seconds = 2
    total_attempts = 0

    while status == "processing":
        total_attempts += 1
        if total_attempts > 10:
            raise RuntimeError("failed to parse document")

        response = requests.get(url, headers=headers)
        response.raise_for_status()  # good practice
        status = response.json().get("status")
        print(status)

        time.sleep(wait_seconds)
        wait_seconds *= 2  # exponential backoff

    return response.json()

print(poll_job(job_id))
```

Once done, you know your document is ready and you can now standardize it using a Schema.

> 📘
>
> You can avoid polling altogether by registering a webhook, which notifies you as soon as parsing or standardization completes. Learn more in the [webhooks guide](https://docs.docupipe.ai/reference/webhooks).

## Building a Schema   [Skip link to Building a Schema](https://docs.docupipe.ai/reference/getting-started-with-docupipe\#building-a-schema)

You can define a schema with code, but it's usually easier to do this part interactively from your dashboard. Select one or more example documents and describe, in plain text, exactly what you want to extract.

Here's an example: go to the Documents tab and select your document.

![](https://files.readme.io/9463358aa14d534db712ec9109b9569a5fc8a4d95302d01ae541510163e7a2d3-image.png)

Click **Create Schema** and type instructions for how you want to understand rental leases.

You can be extremely thorough. For this demo we'll keep things intentionally short: "Extract the renter information and the lease terms. Extract nothing else."

![](https://files.readme.io/67a65f619d1984577d168d845a08e0c08c8c007dee26e7c2933b49683a1d5057-image.png)

Click **Next** and submit. After a short while you will get a schema that defines the slots for extraction. Click any schema to inspect or edit it.

![](https://files.readme.io/987464f6e0435d20ee2572b93472adc0db114fface3ce02d8df9910b650bfde9-image.png)

Now let's use this schema to extract information from documents.

## Standardizing a Document Using a Schema   [Skip link to Standardizing a Document Using a Schema](https://docs.docupipe.ai/reference/getting-started-with-docupipe\#standardizing-a-document-using-a-schema)

First, you need to make a [Standardize](https://docs.docupipe.ai/reference/post_standardize_batch_v2) request, this takes in your **doucment id** and your **schema id**:

PythonJavaScript

```python
HEADERS = {
    "accept": "application/json",
    "X-API-Key": "YOUR_API_KEY"
}

def standardize_batch(doc_ids, schema_id):
    """Standardize a batch of documents."""
    url = f"https://app.docupipe.ai/v2/standardize/batch"
    payload = {"schemaId": schema_id, "documentIds": doc_ids}
    response = requests.post(url, json=payload, headers=HEADERS)
    assert response.status_code == 200
    return {"jobId": res_json["jobId"], "standardizationIds": res_json["standardizationIds"]}

json_response = standardize_batch(['exampleDocumenId'], 'schema_id)
```

```javascript
HEADERS = {
    "accept": "application/json",
    "X-API-Key": "YOUR_API_KEY"
}

async function standardizeBatch(docIds, schemaId) {
  const url = "https://app.docupipe.ai/v2/standardize/batch";
  const payload = { schemaId, documentIds: docIds };

  const response = await fetch(url, {
    method: "POST",
    headers: HEADERS,
    body: JSON.stringify(payload)
  });

  if (!response.ok) throw new Error("Request failed");

  const resJson = await response.json();
  return {
    jobId: resJson.jobId,
    standardizationIds: resJson.standardizationIds
  };
}

standardizeBatch(["exampleDocumentId"], "schema_id");
```

This will give back a payload with `jobId`. Poll for its completion as before - the job will name a `standardizationId` which lets you get the result, once the job is in a `completed` state. Then finally call [Retrieve a Standardization](https://docs.docupipe.ai/reference/get_standardization-1)

PythonJavaScript

```python
HEADERS = {
    "accept": "application/json",
    "X-API-Key": "YOUR_API_KEY"
}
std_id = json_response['standardizationIds'][0]

def get_std(std_id):
    """Retrieve standardized document results from DocuPipe."""
    url = f"{APP_URL}/standardization/{std_id}"
    response = requests.get(url, headers=HEADERS)
    if response.status_code == 200:
        return response.json()
    return None

print(get_std(std_id))
```

```javascript
const HEADERS = {
  accept: "application/json",
  "X-API-Key": "YOUR_API_KEY"
};

const stdId = jsonResponse.standardizationIds[0];

async function getStd(stdId) {
  const url = `${APP_URL}/standardization/${stdId}`;
  const response = await fetch(url, { headers: HEADERS });

  if (response.ok) {
    return await response.json();
  }
  return null;
}

getStd(stdId).then(console.log);
```

This will print the standardization payload - simply a JSON that contains all the things we asked for in our schema.

```undefined
{
  "renterInformation": {
    "tenantName": "Silvia Mando",
    "rentalAddress": {
      "street": "9876 Cherry Avenue, Apartment 426",
      "city": null,
      "state": null,
      "zip": null
    }
  },
  "leaseTerms": {
    "agreementDate": "2012-06-15",
    "leaseType": "Fixed-Term",
    "leaseStartDate": "2012-07-01",
    "leaseEndDate": "2013-06-30",
    "leaseDuration": "one year",
    "monthlyRent": 685,
    "rentCurrency": "USD",
    "rentDueDay": 1,
    "securityDeposit": 685,
    "lateFeeGracePeriod": 3,
    "lateFeeInitial": 25,
    "lateFeeDaily": 5,
    "badCheckFee": 25,
    "cleaningFee": 200,
    "maxVehicles": 1,
    "petRentMonthly": 25,
    "petsAllowed": true
  }
}
```

Using our schema creation dashboard, you can create very complex schemas that are specific to your use case. You can add an exact field for an annual payment for a rental contract, or have a field to describe whether tenants are likely allowed to keep a pet crocodile in the house. Schemas let you understand documents in a way that can be entirely unique to your use case.

There's plenty more to explore with the DocuPipe API:

1. [Classify](https://docs.docupipe.ai/reference/post_classify_batch-1) documents by type so you can route them to the right schema downstream.
2. [Split](https://docs.docupipe.ai/reference/post_split-1) long documents into smaller sub-documents using AI to decide where one ends and the next begins.
3. Generate a [visual review](https://docs.docupipe.ai/reference/post_review_batch-1) of any standardization to see the exact pixels that justify each prediction.
4. Use [Workflows](https://docs.docupipe.ai/reference/post_workflow_on_submit_document-1) to automate sequences such as upload -> classify -> standardize in a single call. See the [workflow code sample](https://docupipe.readme.io/reference/workflow-upload-classify-and-standardize) for details.
5. Use [Webhooks](https://docupipe.readme.io/reference/webhooks) to receive results as soon as they're ready instead of polling.

Updated5 months ago

* * *

Did this page help you?

Yes

No

- [Upload Documents](https://docs.docupipe.ai/reference/getting-started-with-docupipe#upload-documents)
  - [Posting a Document with Code](https://docs.docupipe.ai/reference/getting-started-with-docupipe#posting-a-document-with-code)
  - [Polling for Upload Job Completion](https://docs.docupipe.ai/reference/getting-started-with-docupipe#polling-for-upload-job-completion)
- [Building a Schema](https://docs.docupipe.ai/reference/getting-started-with-docupipe#building-a-schema)
- [Standardizing a Document Using a Schema](https://docs.docupipe.ai/reference/getting-started-with-docupipe#standardizing-a-document-using-a-schema)