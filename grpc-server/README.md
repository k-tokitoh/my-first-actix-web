# run

```
cargo run --bin helloworld-server
```

# example

## grpcurl

```
brew intall grpcurl
```

```
% grpcurl -plaintext -import-path ./proto -proto my_first_tonic.proto -d '{"name": "Tonic"}' '[::1]:50051' my_first_tonic.Greeter/SayHello
{
  "message": "Hello Tonic!"
}
```

## rust bin

```
% cargo run --bin helloworld-client
   Compiling grpc-server v0.1.0 (/Users/takashi/src/github.com/k-tokitoh/my-first-actix-web/grpc-server)
    Finished dev [unoptimized + debuginfo] target(s) in 5.49s
     Running `target/debug/helloworld-client`
RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Mon, 17 Jul 2023 06:52:08 GMT", "grpc-status": "0"} }, message: HelloReply { message: "Hello k-tokitoh!" }, extensions: Extensions }
```
