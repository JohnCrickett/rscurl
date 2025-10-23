# Rust Curl
Rust solution to Coding Challenges [build your own curl](https://codingchallenges.fyi/challenges/challenge-curl/)

## Testing
As this is a complete solution, the intermediate build steps aren't shown.


### Get
```bash
cargo run -- http://eu.httpbin.org:80/get
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/rscurl 'http://eu.httpbin.org:80/get'`
{
  "args": {}, 
  "headers": {
    "Accept": "*/*", 
    "Host": "eu.httpbin.org", 
    "User-Agent": "rust-client", 
    "X-Amzn-Trace-Id": "Root=1-68f94415-6863c0f543a4a60d4d43783d"
  }, 
  "origin": "84.69.23.61", 
  "url": "http://eu.httpbin.org/get"
}

```

### Get With Verbose
### Get
```bash
cargo run --  -v  http://eu.httpbin.org:80/get
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/rscurl -v 'http://eu.httpbin.org:80/get'`
> GET /get HTTP/1.1
> Host: eu.httpbin.org
> Accept: */*
>
< HTTP/1.1 200 OK
< Date: Wed, 22 Oct 2025 20:53:29 GMT
< Content-Type: application/json
< Content-Length: 258
< Connection: close
< Server: gunicorn/19.9.0
< Access-Control-Allow-Origin: *
< Access-Control-Allow-Credentials: true
<
{
  "args": {}, 
  "headers": {
    "Accept": "*/*", 
    "Host": "eu.httpbin.org", 
    "User-Agent": "rust-client", 
    "X-Amzn-Trace-Id": "Root=1-68f94448-70fc5a6865883f047a17dea1"
  }, 
  "origin": "84.69.23.61", 
  "url": "http://eu.httpbin.org/get"
}
```

### Delete
```bash
cargo run -- -X DELETE http://eu.httpbin.org/delete
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/rscurl -X DELETE 'http://eu.httpbin.org/delete'`
{
  "args": {}, 
  "data": "", 
  "files": {}, 
  "form": {}, 
  "headers": {
    "Accept": "*/*", 
    "Host": "eu.httpbin.org", 
    "User-Agent": "rust-client", 
    "X-Amzn-Trace-Id": "Root=1-68f9446d-6c75263e733dcc3c5ea70f9d"
  }, 
  "json": null, 
  "origin": "84.69.23.61", 
  "url": "http://eu.httpbin.org/delete"
}


```
### POST
```bash
cargo run -- -X POST -d '{"key": "value"}' -H "Content-Type: application/json" http://eu.httpbin.org/post
% cargo run -- -X POST -d '{"key": "value"}' -H "Content-Type: application/json" http://eu.httpbin.org/post 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/rscurl -X POST -d '{"key": "value"}' -H 'Content-Type: application/json' 'http://eu.httpbin.org/post'`
connecting to: http://eu.httpbin.org/post
{
  "args": {}, 
  "data": "{\"key\": \"value\"}", 
  "files": {}, 
  "form": {}, 
  "headers": {
    "Accept": "*/*", 
    "Content-Length": "16", 
    "Content-Type": "application/json", 
    "Host": "eu.httpbin.org", 
    "User-Agent": "rscurl", 
    "X-Amzn-Trace-Id": "Root=1-68faa1f0-48ed23dd27c14cd60aa19cc0"
  }, 
  "json": {
    "key": "value"
  }, 
  "origin": "84.69.23.61", 
  "url": "http://eu.httpbin.org/post"
}
```


### PUT
```bash
cargo run -- -X PUT -d '{"key2": "value2"}' -H "Content-Type: application/json" http://eu.httpbin.org/put

```






