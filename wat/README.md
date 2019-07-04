# Build

Use [wat2wasm](https://github.com/WebAssembly/wabt) to convert your .wat file to .wasm files

`wat2wasm add.wat`

# Serve

Use any web server to serve all assets and preview `index.html`. 

I'm using [http-serve](https://github.com/dkarmalita/http-serve)

`http-serve -p 8080 .`