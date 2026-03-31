import { publicProcedure } from "../index";
import {
  loadProductContext,
  buildStoryMap,
  buildProgress,
  buildModuleGraph,
} from "../lib/product-context-loader";

export const productContextRouter = {
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
