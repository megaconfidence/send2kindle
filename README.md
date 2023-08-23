# send2kindle
This project is an OSS microservice written in Rust to send webpages to Kindle
devices.

> To use this service, it's important you add the sender email to receive
> documents on your Kindle device. A guide on how to do this can be found
> [here](https://www.amazon.com/gp/help/customer/display.html?nodeId=GX9XLEVV8G4DB28H).

## Usage 
```sh 
git clone https://github.com/megaconfidence/send2kindle.git 
cd send2kindle
docker build . -t megaconfidence/send2kindle
docker compose up -d
```

## Dependencies
Project dependencies:
* [Open SSL](https://github.com/openssl/openssl)
* [Headless Chrome](https://www.google.com/chrome/)


## Routes 
### / 
You can make a get request to the route route `/` to check the server status.

```sh 
curl http://localhost:3310/
```

### /send 
To send a webpage to any kindle email address by making a post request to the 
`/send` endpoint.
```sh 
curl -X POST http://localhost:3310/send \
   -H "Content-Type: application/json" \
   -d '{"email": "your_id@kindle.com", "url": "http://example.com/"}'
```

## Roadmap 
This project is still in active development and would require a few new features
to make is as accessible as possible:
- [ ] Webclient gui 
- [ ] Use of an OSS email server
- [ ] Automated Docker builds 
- [ ] Telemetry 

