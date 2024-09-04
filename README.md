# rust-std-web-server

This is a web server written in [Rust](https://www.rust-lang.org/) that only uses the [std](https://doc.rust-lang.org/std/) library.

In doing so, we are able to statically link and containerize the web server, giving us a Docker image that is ~5 MB in size.

## To-Do
- Add to CI/CD
    - OWASP ZAP
    - SonarCloud
      - SonarCLoud does not support rust but we can use cargo clippy and cargo tarpaulin. cargo audit?
    - cargo test (unit testing)
- Rust: things not in std: log, regex, tls
- enable binding to both IPv4 and IPv6 in main.rs
- convert all current print statements into logs (4/2024 - can't do this in native rust. only println exists)
- tls/https
- thread pool instead of new thread for each connection (ask copilot)
  - consider changing the type alias 'Job' to 'Task' in the thread pool benchmark
- as per https://en.wikipedia.org/wiki/HTTP#Response_status_codes, make the reason phrases overridable. provide defaults but allow overriding.
- change the tcp_stream.read to tcp_stream.read_to_end now that we have a timeout
  - scratch the above. use bufreader instead
- look into if it's best practice to have main() return Result<(), Box<dyn Error>> or std::io::Result<()>
- be sure we are handling header injection attacks when ingesting HTTP requests
- double check all `pub` items. Only make public what needs to be public. [PoLP](https://en.wikipedia.org/wiki/Principle_of_least_privilege)
- eventually I'd like to support all HTTP versions and methods, therefor eliminating the need to do an additional check on if it's supported after I've already checked if it's valid.
- we may have to make a check for version and method. For example, if it's an `HTTP/1.0` request, `OPTION` is not a valid method. Things like that.

## How to run the server

First, navigate into the server folder using `cd server`

- If `const DOCKER: bool = false;` in src/main.rs, run
    ```
    cargo run
    ```

    or potentially

    ```
    sudo cargo run
    ```

    on macOS or Linux

- If `const DOCKER: bool = true;` in src/main.rs, 
    - First, ensure Docker desktop is running
    - Then, run
        ```
        docker build . -t <image-name>
        docker run -p 8000:8000 <image-name>
        ```

Last, regardless of which step you took above, navigate to `127.0.0.1:8000` in your browser.

### Run on Azure App Service

To run on an Azure App Service, ensure that there is an environment variable (App Setting) set to the following:

```
WEBSITES_PORT=8000
```

## Goal
To build a rust web server (not framework) that uses only the base rust crates, no external crates. Call it mesh? We should allow others to fork or clone it (or maybe use a GH Action?) to utilize it. They just drop their site in and run it. Preferably Docker. Claim to fame: smallest Docker images you can get.

## Known issues
The Docker build does not seem to work on Apple silicon as I'm not sure there is a way to compile static rust binaries on apple silicon. I'm thinking [this is the issue](https://stackoverflow.com/questions/76618704/is-it-possible-to-static-link-the-rust-application-when-using-apple-m1).

## Explanation of decisions
1. No uses of `panic!` or `expect`, instead we use graceful error handling with `Result` as it is more appropriate for production level code

1. We use the `?` operator to propagate errors (when possible) instead of match statements as they are more concise and easier to read.

1. When creating the `TcpListener` in main, we explicitly use a `SocketAddr` (as oppose to just providing a string) as it is the more explicit approach. If you provide a string (&str) or other allowed value, it will create and use a `SocketAddr` behind the scenes anyway, so let's bring that logic to the front and make it visible. [Test](server/src/main.rs#L20)

1. To the best of our ability, we follow the https://en.wikipedia.org/wiki/Principle_of_least_astonishment. Meaning that our functions and variables should do exactly what they seem like they would do on the surface. Ex. a function like `is_valid_http_request_method` returns a boolean, not a string of the validated HTTP method. But a function like `validate_http_request_method` might validate AND return the valid HTTP method.


## Decision Explanations for my future self

### http/mod.rs
1. In files like http.rs, we use naming conventions that may seem repetitive (like using `HttpRequest` inside of the `http.rs` file as oppose to just using `Request`) because I prefer, in functions like `vec_u8_to_http_request` to have the return type be `HttpRequest` as the function name implies

1. In structs like those in `http.rs`, we use `String` as oppose to `&str` because

1. We validate the individual components of a received HttpRequest as we construct it (instead of validating after construction) because validating components as you parse them allows for early error detection and more efficient resource usage (i.e., avoiding constructing and then discarding an invalid HttpRequest.).

1. We use [match guards](https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html) when parsing the received lines so we can validate the data as we structure it.

1. We use `const` and not `static` for the supported methods and versions because they are immutable and known at compile time.

1. we choose a vec<u8> for the httpmessage body because "the body of an HTTP request can contain any type of data, including binary data, which is best represented as a sequence of bytes." (Copilot)

1. According to copilot, `.contains` is case sensitive

1. I'm refactoring to not use .lines or .split_whitespace because it ignores trailing and leading whitespace. Technically trialing and leading whitespace should return an error so I am going to follow the spec as much as possible. (need a ref for this maybe? link to the spec)

1. We should be parsing the raw bytes as oppose to converting it to a string first (according to copilot ->) because that approach is more efficient and allows for better control over the parsing process, especially when dealing with potentially malformed or malicious input.

1. The types for all elements of both `RequestLine` and `StatusLine` will be `Vec<u8>` to adhere to uniformity across our implementation. Using `[u8; x]`, although more exact, lead to differing, more complex code with no real performance gain. (maybe give an example of how the code is more complex?). From Copilot: by using `Vec<u8>` we are "avoiding the complexity of lifetimes and the rigidity of fixed-size arrays."