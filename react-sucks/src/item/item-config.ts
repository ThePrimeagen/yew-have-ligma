export type ItemConfig = {
    girth: number;
    depth: number;
    buffer: Uint8Array;
    bit: number;
}

export default function hasBackground({buffer, bit}: ItemConfig): boolean {
    const offsetByte = Math.floor(bit / 8);
    const offsetBit = 0x1 << (Math.max(0, (bit % 8) - 1));
    return Boolean(buffer[offsetByte] & offsetBit);
}

