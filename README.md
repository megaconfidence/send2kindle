# send2kindle
This project is an OSS microservice written in Rust to send webpages to Kindle
devices.

## Dependencies
Project dependencies:
* [Open SSL](https://github.com/openssl/openssl) i.e `libssl-dev` on Ubuntu or `openssl-devel` on Fedora
* Headless [Chrome](https://www.google.com/chrome/)


## Routes 
### / 
You can make a get request to the route route `/` to check the server status.

```sh 
curl http://localhost:3000/
```

### /send 
To send a webpage to any kindle email address by making a post request to the 
`/send` endpoint.
```sh 
curl -X POST http://localhost:3000/send \
   -H "Content-Type: application/json" \
   -d '{"email": "your_id@kindle.com", "url": "http://example.com/"}'
```
