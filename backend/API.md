# Routes

This document contains the planned routes for the server API

- Home (/)
  - This contains the single page web app 
- Auth **POST** (/api/auth)
- Check Token **POST** (/api/auth/check)
- List Drives **GET** (/api/drives)
- Mount Drive **POST** (/api/drives)
- Unmount Drive **DELETE** (/api/drives/{drive})
- List Files **GET** (/api/files/{drive}/{folder_path})
- View File **GET** (/api/files/{drive}/{folder_path}/${file})
- Delete File/Folder **DELETE** (/api/files/{drive}/{path})
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