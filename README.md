## To-Do
- Add to CI/CD
    - OWASP ZAP
    - SonarCloud
- Rust: things not in std: log, regex, tls
- enable binding to both IPv4 and IPv6 in main.rs
- convert all current print statements into logs (4/2024 - can't do this in native rust. only println exists)
- tls/https
- thread pool instead of new thread for each connection (ask copilot)
  - consider changing the type alias 'Job' to 'Task' in the thread pool benchmark

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
        docker run -p 80:80 <image-name>
        ```

Last, regardless of which step you took above, navigate to `127.0.0.1:80` in your browser.


## Goal
To build a rust web server (not framework) that uses only the base rust crates, no external crates. Call it mesh? We should allow others to fork or clone it (or maybe use a GH Action?) to utilize it. They just drop their site in and run it. Preferrably docker. Claim to fame: smallest docker images you can get.

## Known issues
The docker build does not seem to work on apple silicon as im not sure there is a way to compile static rust binaries on apple silicon. I'm thinking [this is the issue](https://stackoverflow.com/questions/76618704/is-it-possible-to-static-link-the-rust-application-when-using-apple-m1).