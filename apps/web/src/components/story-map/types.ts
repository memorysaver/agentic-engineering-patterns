export type Card = {
  storyId: string;
  title: string;
  moduleId: string;
  layer: number;
  slice: number;
  status: string;
  complexity?: string;
  priority?: string;
  businessValue?: string;
  dependencies: string[];
  prUrl?: string | null;
  completedAt?: string | null;
  attemptCount: number;
};

export type Edge = { from: string; to: string; fromStatus: string };
export type BackboneItem = { id: string; name: string; package?: string; status?: string };
export type Lane = { id: string; name: string; theme?: string; capabilities?: string[] };
export type Slice = { slice: number; theme: string; storyIds: string[] };
