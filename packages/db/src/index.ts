import * as schema from "./schema";
import { drizzle } from "drizzle-orm/d1";
import { env } from "@agentic-engineering-patterns/env/server";

export function createDb() {
  return drizzle(env.DB, { schema });
}
