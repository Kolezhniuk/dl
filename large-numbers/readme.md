# Large numbers

## Task

1. Using one of the existing libraries (you can do it yourself, but it is still better to use the existing one to save time), output the number of key options that can be set 8-, 16-, 32-, 64-, 128-, 256-, 512-, 1024-, 2048-, 4096-bit sequence.
2. Example: If the key length is 16 bits, then the key space is 65,536.
The key space is the number of unique keys that are in a given range.
3. For each of the options, it is necessary to generate a random key value, which is in the range 0x00...0 to 0xFF...F depending on the selected key length.

## Run code

`node ./large-numbers/index.js`

## Implementation Notes

- Language: JavaScript
- Runs both NodeJS v16.15.0 or above
- results print out to console
- big integers creates from bytes in little endian
- to run in browser just comment first line and include index.js file to HTML
