[Skip to content](https://docs.paperless-ngx.com/#features)

[![Paperless-ngx](https://docs.paperless-ngx.com/assets/logo.svg)](https://docs.paperless-ngx.com/ "Paperless-ngx")

Paperless-ngx



Home



Search


[paperless-ngx/paperless-ngx](https://github.com/paperless-ngx/paperless-ngx "Go to repository")

- [Home](https://docs.paperless-ngx.com/)
- [Setup](https://docs.paperless-ngx.com/setup/)
- [Basic Usage](https://docs.paperless-ngx.com/usage/)
- [Configuration](https://docs.paperless-ngx.com/configuration/)
- [Administration](https://docs.paperless-ngx.com/administration/)
- [Advanced Topics](https://docs.paperless-ngx.com/advanced_usage/)
- [REST API](https://docs.paperless-ngx.com/api/)
- [Development](https://docs.paperless-ngx.com/development/)
- [FAQs](https://docs.paperless-ngx.com/faq/)
- [Troubleshooting](https://docs.paperless-ngx.com/troubleshooting/)
- [Changelog](https://docs.paperless-ngx.com/changelog/)

[![Paperless-ngx](https://docs.paperless-ngx.com/assets/logo.svg)](https://docs.paperless-ngx.com/ "Paperless-ngx")
Paperless-ngx


[paperless-ngx/paperless-ngx](https://github.com/paperless-ngx/paperless-ngx "Go to repository")

-

Home



[Home](https://docs.paperless-ngx.com/)
On this page


  - [Features](https://docs.paperless-ngx.com/#features)
  - [Paperless, a history](https://docs.paperless-ngx.com/#paperless-a-history)
  - [Screenshots](https://docs.paperless-ngx.com/#screenshots)
  - [Support](https://docs.paperless-ngx.com/#support)

    - [Feature Requests](https://docs.paperless-ngx.com/#feature-requests)
    - [Bugs](https://docs.paperless-ngx.com/#bugs)

  - [Contributing](https://docs.paperless-ngx.com/#contributing)

    - [Translation](https://docs.paperless-ngx.com/#translation)

  - [Scanners & Software](https://docs.paperless-ngx.com/#scanners-software)

- [Setup](https://docs.paperless-ngx.com/setup/)
- [Basic Usage](https://docs.paperless-ngx.com/usage/)
- [Configuration](https://docs.paperless-ngx.com/configuration/)
- [Administration](https://docs.paperless-ngx.com/administration/)
- [Advanced Topics](https://docs.paperless-ngx.com/advanced_usage/)
- [REST API](https://docs.paperless-ngx.com/api/)
- [Development](https://docs.paperless-ngx.com/development/)
- [FAQs](https://docs.paperless-ngx.com/faq/)
- [Troubleshooting](https://docs.paperless-ngx.com/troubleshooting/)
- [Changelog](https://docs.paperless-ngx.com/changelog/)

# Home

![image](https://docs.paperless-ngx.com/assets/logo_full_black.svg#only-light)![image](https://docs.paperless-ngx.com/assets/logo_full_white.svg#only-dark)

**Paperless-ngx** is a _community-supported_ open-source document management system that transforms your
physical documents into a searchable online archive so you can keep, well, _less paper_.

[Get started](https://docs.paperless-ngx.com/setup/) [Demo](https://demo.paperless-ngx.com/)

![](https://opensource.nyc3.cdn.digitaloceanspaces.com/attribution/assets/PoweredByDO/DO_Powered_by_Badge_white.svg#only-dark)![](https://opensource.nyc3.cdn.digitaloceanspaces.com/attribution/assets/PoweredByDO/DO_Powered_by_Badge_black.svg#only-light)

![image](https://docs.paperless-ngx.com/assets/screenshots/documents-smallcards.png#only-light)![image](https://docs.paperless-ngx.com/assets/screenshots/documents-smallcards-dark.png#only-dark)

## Features

- **Organize and index** your scanned documents with tags, correspondents, types, and more.
- _Your_ data is stored locally on _your_ server and is never transmitted or shared in any way.
- Performs **OCR** on your documents, adding searchable and selectable text, even to documents scanned with only images.
- Utilizes the open-source Tesseract engine to recognize more than 100 languages.
- Documents are saved as PDF/A format which is designed for long term storage, alongside the unaltered originals.
- Uses machine-learning to automatically add tags, correspondents and document types to your documents.
- Supports PDF documents, images, plain text files, Office documents (Word, Excel, PowerPoint, and LibreOffice equivalents)\[^1\] and more.
- Paperless stores your documents plain on disk. Filenames and folders are managed by paperless and their format can be configured freely with different configurations assigned to different documents.
- **Beautiful, modern web application**that features:
  - Customizable dashboard with statistics.
  - Filtering by tags, correspondents, types, and more.
  - Bulk editing of tags, correspondents, types and more.
  - Drag-and-drop uploading of documents throughout the app.
  - Customizable views can be saved and displayed on the dashboard and / or sidebar.
  - Support for custom fields of various data types.
  - Shareable public links with optional expiration.
- **Full text search**helps you find what you need:
  - Auto completion suggests relevant words from your documents.
  - Results are sorted by relevance to your search query.
  - Highlighting shows you which parts of the document matched the query.
  - Searching for similar documents ("More like this")
- **Email processing**\[^1\]: import documents from your email accounts:
  - Configure multiple accounts and rules for each account.
  - After processing, paperless can perform actions on the messages such as marking as read, deleting and more.
- A built-in robust **multi-user permissions** system that supports 'global' permissions as well as per document or object.
- A powerful workflow system that gives you even more control.
- **Optimized** for multi core systems: Paperless-ngx consumes multiple documents in parallel.
- The integrated sanity checker makes sure that your document archive is in good health.

\[^1\]: Office document and email consumption support is optional and provided by Apache Tika (see [configuration](https://docs.paperless-ngx.com/configuration/#tika))

## Paperless, a history

Paperless-ngx is the official successor to the original [Paperless](https://github.com/the-paperless-project/paperless) & [Paperless-ng](https://github.com/jonaswinkler/paperless-ng) projects and is designed to distribute the responsibility of advancing and supporting the project among a team of people. [Consider joining us!](https://github.com/paperless-ngx/paperless-ngx#community-support)

Further discussion of the transition between these projects can be found at
[ng#1599](https://github.com/jonaswinkler/paperless-ng/issues/1599) and [ng#1632](https://github.com/jonaswinkler/paperless-ng/issues/1632).

## Screenshots

Paperless-ngx aims to be as nice to use as it is useful. Check out some screenshots below.

![image](https://docs.paperless-ngx.com/assets/screenshots/dashboard.png)

The dashboard shows saved views which can be sorted. Documents can be uploaded with the button or dropped anywhere in the application.

The document list provides three different styles to browse your documents.

![image](https://docs.paperless-ngx.com/assets/screenshots/documents-table.png)![image](https://docs.paperless-ngx.com/assets/screenshots/documents-smallcards.png)![image](https://docs.paperless-ngx.com/assets/screenshots/documents-largecards.png)

Use the 'slim' sidebar to focus on your docs and minimize the UI.

![image](https://docs.paperless-ngx.com/assets/screenshots/documents-smallcards-slimsidebar.png)

Of course, Paperless-ngx also supports dark mode:

![image](https://docs.paperless-ngx.com/assets/screenshots/documents-smallcards-dark.png)

Quickly find documents with extensive filtering mechanisms.

![image](https://docs.paperless-ngx.com/assets/screenshots/documents-filter.png)

And perform bulk edit operations to set tags, correspondents, etc. as well as permissions.

![image](https://docs.paperless-ngx.com/assets/screenshots/bulk-edit.png)

Side-by-side editing of documents.

![image](https://docs.paperless-ngx.com/assets/screenshots/editing.png)

Support for custom fields.

![image](https://docs.paperless-ngx.com/assets/screenshots/custom_field1.png)

![image](https://docs.paperless-ngx.com/assets/screenshots/custom_field2.png)

A robust permissions system with support for 'global' and document / object permissions.

![image](https://docs.paperless-ngx.com/assets/screenshots/permissions_global.png)

![image](https://docs.paperless-ngx.com/assets/screenshots/permissions_document.png)

Searching provides auto complete and highlights the results.

![image](https://docs.paperless-ngx.com/assets/screenshots/search-preview.png)

![image](https://docs.paperless-ngx.com/assets/screenshots/search-results.png)

Tag, correspondent, document type and storage path editing.

![image](https://docs.paperless-ngx.com/assets/screenshots/new-tag.png)![image](https://docs.paperless-ngx.com/assets/screenshots/new-correspondent.png)![image](https://docs.paperless-ngx.com/assets/screenshots/new-document_type.png)![image](https://docs.paperless-ngx.com/assets/screenshots/new-storage_path.png)

Mail rules support various filters and actions for incoming e-mails.

![image](https://docs.paperless-ngx.com/assets/screenshots/mail-rules-edited.png)

Workflows provide finer control over the document pipeline and trigger actions.

![image](https://docs.paperless-ngx.com/assets/screenshots/workflow.png)

Mobile devices are supported.

![image](https://docs.paperless-ngx.com/assets/screenshots/mobile1.png)![image](https://docs.paperless-ngx.com/assets/screenshots/mobile2.png)![image](https://docs.paperless-ngx.com/assets/screenshots/mobile3.png)

## Support

Community support is available via [GitHub Discussions](https://github.com/paperless-ngx/paperless-ngx/discussions/) and [the Matrix chat room](https://matrix.to/#/#paperless:matrix.org).

### Feature Requests

Feature requests can be submitted via [GitHub Discussions](https://github.com/paperless-ngx/paperless-ngx/discussions/categories/feature-requests) where you can search for existing ideas, add your own and vote for the ones you care about.

### Bugs

For bugs please [open an issue](https://github.com/paperless-ngx/paperless-ngx/issues) or [start a discussion](https://github.com/paperless-ngx/paperless-ngx/discussions/categories/support) if you have questions.

## Contributing

People interested in continuing the work on paperless-ngx are encouraged to reach out on [GitHub](https://github.com/paperless-ngx/paperless-ngx) or [the Matrix chat room](https://matrix.to/#/#paperless:matrix.org). If you would like to contribute to the project on an ongoing basis there are multiple teams (frontend, ci/cd, etc) that could use your help so please reach out!

### Translation

Paperless-ngx is available in many languages that are coordinated on [Crowdin](https://crowdin.com/project/paperless-ngx). If you want to help out by translating paperless-ngx into your language, please head over to the [Paperless-ngx project at Crowdin](https://crowdin.com/project/paperless-ngx), and thank you!

## Scanners & Software

Paperless-ngx is compatible with many different scanners and scanning tools. A user-maintained list of scanners and other software is available on [the wiki](https://github.com/paperless-ngx/paperless-ngx/wiki/Scanner-&-Software-Recommendations).

Back to top
[Next\\
\\
\\
Setup](https://docs.paperless-ngx.com/setup/)

Copyright © 2016 - 2026 Daniel Quinn, Jonas Winkler, and the Paperless-ngx team





Made with
[Zensical](https://zensical.org/)

[github.com](https://github.com/paperless-ngx/paperless-ngx "github.com")[hub.docker.com](https://hub.docker.com/r/paperlessngx/paperless-ngx "hub.docker.com")[matrix.to](https://matrix.to/#/#paperless:matrix.org "matrix.to")

### Filters

#### Tags