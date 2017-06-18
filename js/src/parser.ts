// parser.ts: zser message parser

import { Varint } from "./varint";

/**
 * Default maximum length of a zser message: 1kB
 * This is conservative as zser's main intended use case is a credential format
 */
const DEFAULT_MAX_LENGTH = 1024;

/** Default maximum depth (i.e. default max level of nested messages) */
const DEFAULT_MAX_DEPTH = 8;

/**
 * Parser for zser messages which invokes callbacks on a given handler object
 * (i.e. this is a non-streaming "push parser" with multi-backend support
 */
export class Parser<T> {
  private remaining: Uint8Array[];

  constructor(
    private readonly handler: IHandler<T>,
    private readonly maxLength = DEFAULT_MAX_LENGTH,
    private readonly maxDepth = DEFAULT_MAX_DEPTH,
  ) {
    this.remaining = [];
  }

  // Parse the given zser message, invoking callbacks as necessary
  public parse(message: Uint8Array) {
    if (message.length > this.maxLength) {
      throw new Error(`maximum message length (${this.maxLength}) exceeded: ${message.length}`);
    }

    this.remaining.push(message);

    if (this.remaining.length > this.maxDepth) {
      throw new Error(`exceeded max depth of nested messages (${this.maxDepth})`);
    }

    // Iterate over the stack of remaining messages, consuming them
    while (this.remaining[this.remaining.length - 1].length > 0) {
      let [id, wiretype] = this.parseFieldPrefix();

      switch (wiretype) {
        case 0:
          this.parseUint64(id);
          break;
        case 2:
          this.parseMessage(id);
          break;
        case 3:
          this.parseBinary(id);
          break;
        default:
          throw new Error(`unknown wiretype: ${wiretype}`);
      }
    }

    this.remaining.pop();
  }

  // Finish parsing, returning the resulting object produced by the builder
  public finish(): T {
    if (this.remaining.length !== 0) {
      throw new Error("attempted to finish without consuming entire message!");
    }

    return this.handler.finish();
  }

  // Pop the top item in the remaining stack and parse a varint from it
  private parseVarint(): [number, Uint8Array] {
    let buffer = this.remaining.pop();

    if (buffer === undefined) {
      throw new Error("buffer underrun");
    }

    return Varint.decode(buffer);
  }

  // Parse the integer each field starts with, extracting field ID and wiretype
  private parseFieldPrefix(): [number, number] {
    let [value, remaining] = this.parseVarint();
    this.remaining.push(remaining);

    let fieldId = value >>> 3;
    let wiretype = value & 0x7;

    return [fieldId, wiretype];
  }

  // Parse a Uint64 value stored as a prefix varint
  private parseUint64(id: number) {
    let [value, remaining] = this.parseVarint();
    this.remaining.push(remaining);
    this.handler.uint64(id, value);
  }

  // Parse a blob of data that begins with a length prefix
  private parseLengthPrefixedData(): Uint8Array {
    let buffer = this.remaining.pop();

    if (buffer === undefined) {
      throw new Error("buffer underrun");
    }

    let [length, remaining] = Varint.decode(buffer);
    let data = remaining.subarray(0, length);
    this.remaining.push(remaining.subarray(length));

    return data;
  }

  // Parse a nested message
  private parseMessage(id: number) {
    this.handler.beginNested();

    let nestedMessage = this.parseLengthPrefixedData();
    this.parse(nestedMessage);
    this.handler.endNested(id);
  }

  // Parse a field containing binary data
  private parseBinary(id: number) {
    let data = this.parseLengthPrefixedData();
    this.handler.binary(id, data);
  }
}

// Callback API used by the parser to process parsed data
export interface IHandler<T> {
  // Called when a uint64 value with the given field ID is parsed
  // TODO: switch to TC39 BigInt when available: https://tc39.github.io/proposal-bigint/
  uint64(id: number, value: number): void;

  // Called when we've received binary data with the given ID
  binary(id: number, data: Uint8Array): void;

  // Indicate we've entered a new nested message
  beginNested(): void;

  // Indicate we've reached the end of a nested message with the given ID
  endNested(id: number): void;

  // Return the fully parsed object
  finish(): T;
}
