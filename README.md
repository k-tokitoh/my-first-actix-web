```mermaid
graph TD;
    curl-->http-server;
    browser-->http-server;
    http-server-->grpc-server;
    grpcurl-->grpc-server;
```
