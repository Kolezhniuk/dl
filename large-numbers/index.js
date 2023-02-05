const crypto = require('crypto').webcrypto;
// all the numbers are in bits
const keyCapacitiesInBits = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096]

for (const capacity of keyCapacitiesInBits) {
    const byteArray = new Uint8Array(capacity / 8) // length in bytes
    const randomBytes = crypto.getRandomValues(byteArray)
    const randomBigInt = getBigInt(randomBytes);
    console.log(`Brute forcing BigInt 0x${randomBigInt.toString(16)}  of given ${capacity}-bit key capacity`);
    console.time(`${capacity}-bit:`);
    for (let i = 0n; i < randomBigInt; i++) { }
    console.timeEnd(`${capacity}-bit:`);
}


function getBigInt(arr) {
    let result = BigInt(0);
    //little-endian
    for (let i = arr.length - 1; i >= 0; i--) {
        // for (let i = 0; i < arr.length; i++) {
        result = result * BigInt(256) + BigInt(arr[i]);
    }
    return result;
}
