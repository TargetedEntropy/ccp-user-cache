# CCP User Cache

A Rust API to provide user details for CCP Users in the popular Space MMO, Eve Online.

This service supports JWT Authentication and Authorization to provide user access to cached API data.  As the official [https://esi.evetech.net/](API) only supports a 30day history, users can access historical data beyond that period.

## Usage

This raw API only support direct HTTP calls, which are for user registration, login, logout and admin functions such as managing users.  There are three levels of access; Admin, Moderator and User.


#### User Registration
To register a user, there are four required fields.

* name
* email
* password
* passwordConfirm

Request:
```
curl -X POST http://localhost:8000/api/auth/register \
   -d '{"name":"Bob Hope","email": "username@gmail.com", "password": "mkFzMYqKtv5Bha6r8GX9DP", "passwordConfirm": "mkFzMYqKtv5Bha6r8GX9DP"}' \
   -H 'Content-Type: application/json'
```

Response:
```
{
   "status":"success",
   "data":{
      "user":{
         "id":"de7d0c80-9a2a-4f71-9e0b-a55c1dfa2f21",
         "name":"Brian",
         "email":"targetedentropy@gmail.com",
         "role":"user",
         "photo":"default.png",
         "verified":false,
         "createdAt":"2024-06-19T14:46:53.667238Z",
         "updatedAt":"2024-06-19T14:46:53.667238Z"
      }
   }
}
```

#### Login
Logging in requires two fields, a successful login returns a token.

* email
* password

Request:
```
curl -X POST http://localhost:8000/api/auth/login \
   -d '{"email": "targetedentropy@gmail.com", "password": "mkFzMYqKtv5Bha6r8GX9DP"}' \
   -H 'Content-Type: application/json'
```

Response:
```
{
    "status":"success",
    "token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkZTdkMGM4MC05YTJhLTRmNzEtOWUwYi1hNTVjMWRmYTJmMjEiLCJpYXQiOjE3MTg4MDkxNzMsImV4cCI6MTcxODgxMjc3M30.oOmUtYzx2pqeAP21G0kWVCJCUYVVlMVjJ7oYeaOwlAA"
}
```

#### Logout
Logging out uses the Session Cookie to identify the user the provides an expired cookie to invalid the original token.

This requires one field in the Cookies.

* token

Request:
```
curl -X POST http://localhost:8000/api/auth/logout \
  --cookie 'token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJkZTdkMGM4MC05YTJhLTRmNzEtOWUwYi1hNTVjMWRmYTJmMjEiLCJpYXQiOjE3MTg4MDkxNzMsImV4cCI6MTcxODgxMjc3M30.oOmUtYzx2pqeAP21G0kWVCJCUYVVlMVjJ7oYeaOwlAA' \
  -H 'Content-Type: application/json'
```

Response:
```
{
    "status":"success"
}
```

## Installation

If you would like to run your own CCP User Cache, you can follow these steps.

* Clone the repo locally
 `git clone https://github.com/TargetedEntropy/ccp-user-cache.git`

* Rename `env.sample` to `.env` and modify the fields.
 `mv env.sample .env`

* Inside the directory, start the Postgres database instances
 `docker-compose -f docker-compose.no_api.yml up -d`

 * Install SQLx
 `cargo install sqlx-cli --no-default-features --features postgres`
 
 * Perform SQL Migrations
 `sqlx migrate run`

 * Start the API
 `cargo run`


