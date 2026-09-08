import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { z } from "zod";

// This adapter validates the transport shape. Rust owns record and readiness rules.
const record = z
  .object({
    id: z.string(),
    kind: z.string(),
    title: z.string(),
    status: z.string(),
    description: z.string(),
    refs: z.array(z.string()),
    depends_on: z.array(z.string()),
    required_gates: z.array(z.string()),
    paths: z.array(z.string()),
    layer: z.string().nullable(),
    wave: z.string().nullable(),
    release: z.string().nullable(),
    change: z.string().nullable(),
    owner: z.string().nullable(),
    data: z.record(z.string(), z.unknown()),
  })
  .passthrough();
const view = z.object({
  schema_version: z.literal(1),
  revision: z.string(),
  records: z.array(record),
  readiness: z.array(
    z.object({ story: z.string(), ready: z.boolean(), reasons: z.array(z.string()) }),
  ),
  diagnostics: z.array(z.object({ code: z.string(), message: z.string() }).passthrough()),
});
export function ledgerRoot(): string | null {
  const root = process.env.AEP_PROJECT_ROOT
    ? resolve(process.env.AEP_PROJECT_ROOT)
    : dirname(resolve(process.env.PRODUCT_CONTEXT_PATH || "./product-context.yaml"));
  return existsSync(resolve(root, ".aep/config.toml")) ? root : null;
}
export function parseLedgerResponse(stdout: string) {
  const result = z
    .object({
      schema_version: z.literal(1),
      ok: z.literal(true),
      exit_code: z.literal(0),
      side_effects: z.literal(false),
      data: view,
    })
    .passthrough()
    .parse(JSON.parse(stdout));
  return result.data;
}
export function loadProjectLedger() {
  const root = ledgerRoot();
  if (!root)
    throw new Error(
      "No AEP project is configured. Set AEP_PROJECT_ROOT to an initialized project.",
    );
  const stdout = execFileSync(
    process.env.AEP_BINARY || "aep",
    ["--root", root, "--json", "dashboard"],
    {
      encoding: "utf8",
      timeout: 15_000,
      maxBuffer: 16 * 1024 * 1024,
      stdio: ["ignore", "pipe", "pipe"],
    },
  );
  return parseLedgerResponse(stdout);
}
