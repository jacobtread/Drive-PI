# Routes

This document contains the planned routes for the server API

- Home (/)
    - This contains the single page web app

### Auth

- Auth **POST** (/api/auth)
- Check Token **POST** (/api/auth/check)

### Drives

- List Drives **GET** (/api/drives)
- Unmount Drive **DELETE** (/api/drives/{drive})

### Files

- List Files **GET** (/api/files/list)
- View File **GET** (/api/files/view)
- Delete File/Folder **DELETE** (/api/files/)

### Settings

- Settings **GET** (/api/settings)
- Set Settings **PUT** (/api/settings)
- Reset Settings **DELETE** (/api/settings)
- Reboot **REBOOT** (/api/reboot)

## AUTH

### Request

```http request
POST /api/auth
Content-Type: application/json

{
    "username": "admin",
    "password": "admin"
}

```

### Response

#### Success

``` json
{
    "token": "HAnQZ701X7VBjuP6sc3pr3ygOmLrqZbs0viBfqGxP6vuX0YC",
    "expiry_time": 1661581641185
}
```

#### Incorrect Credentials

401 Unauthorized

```
invalid credentials
```