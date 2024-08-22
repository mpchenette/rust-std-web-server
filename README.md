# rust-std-web-server

This is a web server written in [Rust](https://www.rust-lang.org/) that only uses the [std](https://doc.rust-lang.org/std/) library.

In doing so, we are able to statically link and containerize the web server, giving us a
Docker image that is ~5 MB in size.

## To-Do
- Add to CI/CD
    - OWASP ZAP
    - SonarCloud
      - SonarCLoud does not support rust but we can use cargo clippy and cargo tarpaulin. cargo audit?
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