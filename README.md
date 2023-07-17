```mermaid
graph TD;
    curl-->client/http-server;
    browser-->client/http-server;
    client/http-server-->server;
    client/rust-bin-->server;
    grpcurl-->server;
```
