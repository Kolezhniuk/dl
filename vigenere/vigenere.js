function vigenereCypher(msg, key) {
    const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let cipherText = "";
    for (let index = 0; index < msg.length; index++) {
        const character = msg[index];
        // (character + key[index % m]) % 26
        const i = (letters.indexOf(character) + letters.indexOf(key[index % msg.length])) % 26
        cipherText += letters[i];
    }
    return cipherText;
}
console.log(
    vigenereCypher("HELLO", "LEMON")
)
console.log(
    vigenereCypher("WORLD", "LEMON")
)
