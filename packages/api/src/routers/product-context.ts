import { publicProcedure } from "../index";
import { ledgerRoot, loadProjectLedger } from "../lib/project-ledger-loader";
import {
  loadProductContext,
  buildStoryMap,
  buildProgress,
  buildModuleGraph,
} from "../lib/product-context-loader";

export const productContextRouter = {
  getMode: publicProcedure.handler(() => ({
    mode: ledgerRoot() ? ("native" as const) : ("legacy" as const),
  })),
  getLedger: publicProcedure.handler(() => loadProjectLedger()),
  getAll: publicProcedure.handler(() => {
    return loadProductContext();
  }),

  getStoryMap: publicProcedure.handler(() => {
    const ctx = loadProductContext();
    return buildStoryMap(ctx);
  }),

  getProgress: publicProcedure.handler(() => {
    const ctx = loadProductContext();
    return buildProgress(ctx);
  }),

  getModuleGraph: publicProcedure.handler(() => {
    const ctx = loadProductContext();
    return buildModuleGraph(ctx);
  }),
};
