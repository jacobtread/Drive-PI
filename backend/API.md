# API Documentation

This resources contains a list of routes that the Drive-PI backend exposes along with example
responses

# Auth Routes

## Response Codes

Below is a table listing response codes for the Auth Routes and what they mean

| Code | What                                      |
|------|-------------------------------------------|
| 401  | Token or credentials were invalid         |
| 400  | Request required token but it was missing |

## Authenticate

This route is for authenticating with a username and password which will provide a
token to authenticate with other routes.

**POST** /api/auth

### Request Body

```json
{
  "username": "admin",
  "password": "admin"
}
```

### Example Response

```json 
{
    "token": "AHDBawiudnabwidbuawdiauydabwyidvvyuawdaw",
    "expire_time": 12321931
 }
```

The token provided by this request should be provided to all other requests through
the X-Token header. The expire_time is the unix time in milliseconds of when the token
becomes invalid.

### Check Authentication

To check the current token information (expire_time and validity) you can use this request

**GET** /api/auth

> Requires X-Token header

### Request Body

*EMPTY REQUEST BODY*

### Example Response

```json
{
  "valid": true,
  "expire_time": 12321931
}
```

The "valid" field determines whether the token is a valid token or not and the expire_time will be null
if the token is not valid or the unix time in milliseconds of when the token will become invalid

### Remove Authentication

To invalidate a token and prevent further use of it you use this request

**DELETE** /api/auth

> Requires X-Token header

### Request Body

*EMPTY REQUEST BODY*

## Drives Routes

### List Drives

You can list both the mounted and unmounted drives using this route

**GET** /api/drives

> Requires X-Token header

### Request Body

*EMPTY REQUEST BODY*

### Example Response

```json
{
  "mount_root": "/usr/dev/drivepi/mount",
  "drives": [
    {
      "uuid": "21c89e37-a0aa-48bc-aead-cec8d9a8e8cc",
      "name": "sda1",
      "label": "USB Drive",
      "path": "/dev/sda1",
      "mount": "/usr/dev/drivepi/mount/USB Drive",
      "size": "10GB",
      "used": "5GB",
      "mode": "brw-rw----"
    },
    {
      "uuid": "21c89e37-a0aa-48bc-aead-cec8d9a8e8cc",
      "name": "sdb1",
      "label": "USB Drive 2",
      "path": "/dev/sdb1",
      "mount": null,
      "size": null,
      "used": null,
      "mode": "brw-rw----"
    }
  ]
}
```

The "mount_root" field is the root path to where shared drives are mounted and is used to determine if a drive is shared based on it's mount path.
Drives where the mount point, size and used are null are not mounted.

### Mount Drive

To mount a drive you can use this route. The drive will be mounted to the Drive-PI share mount root 
using the mount command.

**POST** /api/drives


> Requires X-Token header

### Request Body

Path being the drive path and name being the name of the folder to mount the drive to.

```json
{
    "path": "/dev/sda1",
    "name": "USB Drive"    
}
```

Successful mount is indicated with a 200 status code failures will be present in
the message.

### Unmount

To unmount a drive you can use this route. The drive will be removed from the Drive-PI share mount root
if it exists and unmounted using the umount command.

**DELETE** /api/drives


> Requires X-Token header

### Request Body

Path being the drive path and name being the name of the folder to mount the drive to.

```json
{
    "path": "/dev/sda1",
    "name": "USB Drive"    
}
```

Successful unmount is indicated with a 200 status code failures will be present in
the message.

### Remount

To remount a drive thats mounted outside the Drive-PI mount root you can use this route. The drive will be mounted to the Drive-PI share mount root
using the mount command.

**PUT** /api/drives

> Requires X-Token header

### Request Body

Path being the drive path and name being the name of the folder to mount the drive to.

```json
{
    "path": "/dev/sda1",
    "name": "USB Drive"    
}
```

Successful remount is indicated with a 200 status code failures will be present in
the message.

## Files Routes

You can list the files and folders on a drive using this route.

**POST** /api/files

> Requires X-Token header

### Request Body

Path is the path on the drive relative to the "drive_path" path which should be the mount
root of the drive.

```json
{
    "path": "/folder/on/drive",
    "drive_path": "/usr/bin/drivepi/mount/USB Drive"    
}
```


### Example Response

```json
{
  "folders": [
    {
      "name": "Example Folder",
      "permissions": 644
    },
    {
      "name": "Other Folder",
      "permissions": 777
    }
  ],
  "files": [
    {
      "name": "test.txt",
      "size": 1024,
      "permissions": 644
    }
  ]
}
```