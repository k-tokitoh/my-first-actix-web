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
