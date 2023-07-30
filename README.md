## To-Do
- Add to CI/CD
    - OWASP ZAP
    - SonarCloud
- Rust: things not in std: log, regex, tls
- enable binding to both IPv4 and IPv6 in main.rs
- convert all current print statements into logs

## How to run the server

First, navigate into the server folder using `cd server`

- If `const DOCKER: bool = false;` in src/main.rs, run
    ```
    cargo run
    ```

- If `const DOCKER: bool = true;` in src/main.rs, 
    - First, ensure Docker desktop is running
    - Then, run
        ```
        docker build . -t <image-name>
        docker run -p 80:80 <image-name>
        ```

Last, regardless of which step you took above, navigte to `127.0.0.1:80` in your browser.