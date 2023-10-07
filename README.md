# send2kindle

This project is an OSS service written in Rust to send web content such new
articles and blogs to your Kindle device
devices.

<a href="https://chrome.google.com/webstore/detail/send2kindle/pcnchejipcigndkfoifcnokhiiihfdka" target="_blank">
  <img src="./public/images/chrome.webp" style="height:50px;" />
</a>
<a
  href="https://addons.mozilla.org/en-US/firefox/addon/send2kindle/"
  target="_blank"
>
  <img src="./public/images/firefox.webp" style="height:50px;" />
</a>
</br>
</br>

Project home page [https://send2kindle.confidence.sh/](https://send2kindle.confidence.sh/)

> To use this service, it's important you add the sender email to receive
> documents on your Kindle device. A guide on how to do this can be found
> [here](https://www.amazon.com/gp/help/customer/display.html?nodeId=GX9XLEVV8G4DB28H).

## Dependencies

> The Docker image contains all dependencies required to run this project.

Project dependencies:

- [Rust](https://www.rust-lang.org/tools/install)
- [Open SSL](https://github.com/openssl/openssl)
- [Headless Chrome](https://www.google.com/chrome/)
- [Ghostscript](https://ghostscript.com/docs/9.54.0/Install.htm)

## Local Setup

Instructions below show how to build and run a container for this project.
Please note that you have to create a `.env` file using the variables in this
[guide](./.env.example).

```sh
git clone https://github.com/megaconfidence/send2kindle.git
cd send2kindle
docker build . -t megaconfidence/send2kindle
docker compose up -d
```

Run with `docker run`

```sh
docker run --rm -it --name send2kindle -p 3310:3310 --env-file .env megaconfidence/send2kindle
docker stop send2kindle
```

## Endpoints

### /

This route serves the web client which can be viewed on a browser at:

```sh
http://localhost:3310/
```

### /send

To send a webpage to any kindle email address by making a post request to the
`/send` endpoint.

```sh
curl -X POST http://localhost:3310/send \
-H "Content-Type: application/json" \
-d '{"email": "username@kindle.com", "url": "http://example.com/"}'
```

## Roadmap

This project is still in active development and would require a few new features
to improve accessibility:

- [x] Webclient gui
- [x] Browser extension
- [x] Use of SMTP client
- [x] Add controls for optimized page rendering
- [x] Pdf compression for large webpages
- [x] Automated Docker builds
- [x] Non-blocking background job for rendering and emailing
- [x] Logging/tracing
