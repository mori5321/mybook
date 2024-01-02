# Overview
```mermaid
%%{init: {'theme': 'dark' }}%%
sequenceDiagram
  feature branch->>main branch: Merge PR with content
  main branch->>github artifact: mdbook build & upload /dist directory
  github artifact->>production branch: download /dist directory
  production branch->>cloudflare page: commit /dist directory & deploy it
```
