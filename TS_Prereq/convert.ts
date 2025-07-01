import promptSync from 'prompt-sync';
import bs58 from 'bs58';

const prompt = promptSync();

const phantomBase58 = prompt('Enter Phantom base58 private key: ').trim();

try {
  const walletBytes = bs58.decode(phantomBase58);
  console.log('Decoded byte array:', Array.from(walletBytes));
} catch (err) {
  console.error('Failed to decode base58:', err);
}

const wallet: number[] = [
  81, 137, 204, 28, 201, 124, 188, 121, 4, 194, 6, 121, 182, 238, 179,
  181, 120, 11, 173, 16, 14, 217, 196, 242, 105, 65, 23, 148, 155, 251,
  204, 24, 120, 182, 40, 59, 167, 156, 133, 57, 242, 101, 217, 3, 67, 60,
  175, 12, 185, 93, 136, 214, 127, 50, 19, 243, 45, 240, 80, 102, 112, 192, 99
];

const byteArray = new Uint8Array(wallet);
const solanaBase58 = bs58.encode(byteArray);

console.log('Base58 from Solana byte array:', solanaBase58);
