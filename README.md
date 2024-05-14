# API Documentation

## HTTP Request

### POST /auth/login

Endpoint to authenticate a user.

#### Request

- Method: POST
- URL: `/auth/login`
- Headers:
    - Content-Type: application/json
- Body:
    ```json
    {
        "username": "example",
        "password": "password123"
    }
    ```

#### Response

- Status Code: 200 OK
- Body:
    ```json
    {
        "body": {
            "AuthToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImV4YW1wbGUiLCJpYXQiOjE2MzE0MjEwNzMsImV4cCI6MTYzMTQyNDY3M30.2j7ZLz3Zz0U1w7VJ8JYX3zv6X5e3v9J0Zr2z4z2z4z2"
        }
    }
    ```

#### Error Handling

- Status Code: 400 Bad Request
- Body:
    ```
    Username / Email doesn't exist
    ```



### POST /auth/register

Endpoint to authenticate a user.

#### Request

- Method: POST
- URL: `/auth/register`
- Headers:
    - Content-Type: application/json
- Body:
    ```json
    {
        "username": "example",
        "email": "example@domain.com",
        "password": "password123"
    }
    ```

#### Response

- Status Code: 200 OK
- Body:
    ```json
    {
        "message": "Registration Successfull"
    }
    ```

#### Error Handling

- Status Code: 400 Bad Request
- Body:
    ```
    Username / Email already exist
    ```

### GET /user

Endpoint to get user details.

#### Request

- Method: GET
- URL: `/user`
- Headers:
    - Authorization: Bearer {token}

#### Response

- Status Code: 200 OK
- Body:
    ```json
    {
        "id": 1,
        "username": "example",
        "email": "example@example.com",
        "role": "role",
        "balance": 0
    }
    ```

#### Error Handling

- Status Code: 401 Unauthorized
- Body:
    ```json
    {
        "message": "Unauthorized"
    }
    ```

## gRPC

### AuthService
- verify_role()
- get_claims()
- verify_auth()

### UserService
- update_balance()

