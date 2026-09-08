import { createFileRoute, redirect } from "@tanstack/react-router";
import { client } from "@/utils/orpc";

export const Route = createFileRoute("/")({
  beforeLoad: async () => {
    const { mode } = await client.productContext.getMode();
    throw redirect({ to: mode === "native" ? "/ledger" : "/story-map" });
  },
  component: () => null,
});
