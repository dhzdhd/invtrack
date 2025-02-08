# Invtrack

## API

### AWS Lambda notes

- Local testing
    - URL to make a request using cURL/Postman/etc - `http://localhost:8080/2015-03-31/functions/function/invocations`
    - Add a `application/json` payload
        ```json
        {
            "version": "2.0",
            "routeKey": "$default",
            "rawPath": "/",
            "rawQueryString": "",
            "headers": {},
            "requestContext": {
                "http": {
                "method": "GET",
                "path": "/"
                }
            },
            "isBase64Encoded": false,
            "body": null
        }
        ```
- On AWS
    - Make a request to the provided AWS Lambda URL, no payload required