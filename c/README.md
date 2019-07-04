# Build 
Using [emscripten](https://emscripten.org/)

## To js
`emcc hello.c`

## To asm
`emcc -S hello.c`

## To wasm
`emcc hello.c -o hello.wasm`

## To wasm with html and js glue
`emcc hello.c -o hello.html`

## To js
`emcc hello.c -O3 -o hello-opt.html`