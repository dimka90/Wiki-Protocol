const { rlp, keccak256 , ecsign} = require("ethereumjs-utils");


function deployTransacton(transactionData) {
const payload = JSON.parse(process.argv[2])
const privateKey=Buffer.from(process.argv[3].replace("0x", ""), "hex")
if(privateKey.length !==32 ) {
    console.log("Private key must be 64 characters longs")
    process.exit(1);
}

const unsignedRLP = rlp.encode(payload)

const messageHash = keccak256(unsignedRLP)

const {v,r,s} = ecsign(messageHash,privateKey)
console.log(v)
console.log(r)
console.log(s)

payload.push(
    "0x".concat(v.toString(16)),
    "0x".concat(r.toString("hex")),
    "0x".concat(s.toString("hex"))
);

console.log(rlp.encode(payload).toString("hex"))
}

module.exports =deployTransacton