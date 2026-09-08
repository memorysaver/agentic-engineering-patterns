import { describe, expect, test } from "bun:test";
import { parseLedgerResponse } from "./project-ledger-loader";

describe("native ledger transport", () => {
  const response = {
    schema_version: 1,
    ok: true,
    exit_code: 0,
    side_effects: false,
    data: {
      schema_version: 1,
      revision: "revision",
      records: [],
      diagnostics: [],
      readiness: [{ story: "opaque-4.5", ready: false, reasons: ["Gate is stale"] }],
    },
  };
  test("preserves Rust readiness and opaque IDs", () => {
    expect(parseLedgerResponse(JSON.stringify(response)).readiness).toEqual(
      response.data.readiness,
    );
  });
  test("rejects unsupported and mutating responses", () => {
    expect(() => parseLedgerResponse(JSON.stringify({ ...response, schema_version: 2 }))).toThrow();
    expect(() =>
      parseLedgerResponse(JSON.stringify({ ...response, side_effects: true })),
    ).toThrow();
    expect(() => parseLedgerResponse(JSON.stringify({ ...response, ok: false }))).toThrow();
  });
});
