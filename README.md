# prerequisites

```
% cargo install diesel_cli
% echo DATABASE_URL=sqlite:///path/to/your/db > .env
% diesel setup
% diesel migration run
```

# run

```
cargo run
```

or

```
cargo watch -x run
```

# example

```
my-first-actix-web % curl -X GET -H "Content-Type: application/json" http://127.0.0.1:8080/tasks/
[]%

my-first-actix-web % curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8080/tasks/ --data '{"body": "foo"}'
{"id":3,"body":"foo"}%

my-first-actix-web % curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8080/tasks/ --data '{"body": "bar"}'
{"id":4,"body":"bar"}%

my-first-actix-web % curl -X GET -H "Content-Type: application/json" http://127.0.0.1:8080/tasks/
[{"id":3,"body":"foo"},{"id":4,"body":"bar"}]%

my-first-actix-web % curl -X PUT -H "Content-Type: application/json" http://127.0.0.1:8080/tasks/3 --data '{"body": "hoge"}'
{"id":3,"body":"hoge"}%

my-first-actix-web % curl -X GET -H "Content-Type: application/json" http://127.0.0.1:8080/tasks/
[{"id":3,"body":"hoge"},{"id":4,"body":"bar"}]%

my-first-actix-web % curl -X DELETE -H "Content-Type: application/json" http://127.0.0.1:8080/tasks/4
{"id":4,"body":"bar"}%

my-first-actix-web % curl -X GET -H "Content-Type: application/json" http://127.0.0.1:8080/tasks/
[{"id":3,"body":"hoge"}]%
```
