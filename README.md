# CCP User Cache

A Rust API to provide user details for CCP Users in the popular Space MMO, Eve Online.

This service supports JWT Authentication and Authorization to provide user access to cached API data.  As the official (https://esi.evetech.net/)[API] only supports a 30day history, users can access historical data beyond that period.

## Usage

This raw API only support direct HTTP calls, which are for user registration, login, logout and admin functions such as managing users.  There are three levels of access; Admin, Moderator and User.


### User Registration
To register a user, there are four required fields.

* name
* email
* password
* passwordConfirm

```
curl -X POST http://localhost:8000/api/auth/register 
   -d '{"name":"Bob Hope","email": "username@gmail.com", "password": "mkFzMYqKtv5Bha6r8GX9DP", "passwordConfirm": "mkFzMYqKtv5Bha6r8GX9DP"}'  
    -H 'Content-Type: application/json'
```

### Login


### Logout


## Installation

