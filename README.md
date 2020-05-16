# Todo Points Backend

![Cross-platform CICD](https://github.com/gmackie/todo-points-backend/workflows/Cross-platform%20CICD/badge.svg)
![Docker CICD](https://github.com/gmackie/todo-points-backend/workflows/Docker%20CICD/badge.svg)

A backend server for a todo app that gives you points for completing things. Gameification as a service.

Built off of [SakaDream/actix-web-rest-api-with-jwt](https://github.com/SakaDream/actix-web-rest-api-with-jwt)

## Require

- [Rust Stable](https://rustup.rs)
- [Postgres](https://www.postgresql.org/)

Or using [Docker](https://www.docker.com/)

## How to run

### Manual

- Rename `secret.key.sample` to `secret.key` or create your own key by running `head -c16 /dev/urandom > secret.key` in command line (Linux/UNIX only) and copy to `/src` folder
- Create a database in postgres cli or [pgAdmin](https://www.pgadmin.org/) tool
- Rename `.env.sample` to `.env` and update the database connection string in `DATABASE_URL` key.
- Build with release profile: `cargo build --release`
- Run release binary in command line/terminal.
  - Windows: `target/release/todo-points-backend.exe`
  - Linux/UNIX: `target/release/todo-points-backend`
- Enjoy! 😄

### Docker

- Enter into project directory
- Run `docker-compose -f docker-compose.local.yml up` for local environment or `docker-compose -f docker-compose.prod.yml up` for production environment
- Enjoy! 😄

## APIs

### Address: **`localhost:8000`**

### `GET /api/ping`: Ping
```bash
curl -X GET -i 'http://127.0.0.1:8000/api/ping'
```
- Response:
    - 200 OK
    ```
    pong!
    ```

### `POST /api/auth/signup`: Signup
```bash
curl -X POST -i 'http://127.0.0.1:8000/api/auth/signup' \
  -H "Content-Type: application/json" \
  --data '{
    "username": "user",
    "email": "user@email.com",
    "password": "4S3cr3tPa55w0rd"
  }'
```

  - Request body:
  ```
  {
     "username": string,
     "email": string,
     "password": string       // a raw password
  }
  ```
  - Response
    - 200 OK
    ```
    {
       "message": "signup successfully",
       "data": ""
    }
    ```
    - 400 Bad Request
    ```
    {
       "message": "User '{username}' is already registered",
       "data": ""
    }
    ```

### `POST /api/auth/login`: Login
```bash
curl -X POST -H 'Content-Type: application/json' -i 'http://127.0.0.1:8000/api/auth/login'  \
  --data '{"username_or_email":"user",  "password":"4S3cr3tPa55w0rd"}'
```
  - Request body:
  ```
  {
     "username_or_email": string,
     "password": string       // a raw password
  }
  ```
  - Response
    - 200 OK
    ```
    {
       "message": "login successfully",
       "data": {
         "token": string      // bearer token
       }
    }
    ```
    - 400 Bad Request
    ```
    {
       "message": "wrong username or password, please try again",
       "data": ""
    }
    ```

### `POST /api/auth/logout`: Logout
```bash
curl -X POST -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzcyNTc4NzksImV4cCI6MTU3Nzg2MjY3OSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiYzUxNWE3NTg3NGYzNGVjNGFmNDJmNWE2M2QxMDVjMGYifQ.B9w6FxFdypb5GCRMKXZ9CZWFxQLFjvmPSusMCtcE-Ac' \
  -i 'http://127.0.0.1:8000/api/auth/logout'
```

### brower OPTIONS curl request example
```bash
curl -X OPTIONS -i 'http://127.0.0.1:8000/api/login' \
  -H "Origin: http://example.com" -H "Access-Control-Request-Method: POST"
```
  - Response
  ```
  HTTP/1.1 200 OK
  content-length: 0
  access-control-max-age: 3600
  access-control-allow-methods: POST,DELETE,GET,PUT
  access-control-allow-origin: *
  access-control-allow-headers: authorization,content-type,accept
  date: Tue, 07 Jan 2020 15:17:48 GMT
  ```

### Errors:
  - Invalid or missing token
    - Status code: 401 Unauthorized
    - Response:
    ```
    {
      "message": "invalid token, please login again",
      "data": ""
    }
    ```
