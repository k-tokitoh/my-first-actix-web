```mermaid
graph TD;
    curl-->HTTP_SERVER["client/http-server@actix-web"];
    browser-->HTTP_SERVER;
    HTTP_SERVER-->GRPC_SERVER["server@tonic"];
    client/rust-bin-->GRPC_SERVER;
    grpcurl-->GRPC_SERVER;
```
