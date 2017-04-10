export declare class Varint {
    static readonly MAX: number;
    static readonly CTZ_TABLE: Uint8Array;
    static encode(n: number): Uint8Array;
    static decode(bytes: Uint8Array): number;
}
export declare class Uint64 {
    values: Uint32Array;
    static readonly MAX_SAFE_INTEGER: number;
    static fromNumber(n: number): Uint64;
    static checkInteger(n: number): void;
    constructor(values: Uint32Array);
    lshift(n: number): Uint64;
    rshift(n: number): Uint64;
    bw_or(n: number): Uint64;
    lt_eq(n: number): boolean;
    upper(): number;
    lower(): number;
    toInteger(): number;
}
