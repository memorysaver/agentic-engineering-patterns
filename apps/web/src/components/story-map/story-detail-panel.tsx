import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@agentic-engineering-patterns/ui/components/sheet";
import { Badge } from "@agentic-engineering-patterns/ui/components/badge";
import { Separator } from "@agentic-engineering-patterns/ui/components/separator";
import { getStatusColor, COMPLEXITY_COLORS } from "./status-colors";

type Card = {
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

type StoryDetailPanelProps = {
  card: Card;
  allCards: Card[];
  open: boolean;
  onClose: () => void;
};

export function StoryDetailPanel({ card, allCards, open, onClose }: StoryDetailPanelProps) {
  const colors = getStatusColor(card.status);
  const dependsOn = card.dependencies
    .map((dep) => allCards.find((c) => c.storyId === dep))
    .filter(Boolean) as Card[];
  const blockedBy = allCards.filter((c) => c.dependencies.includes(card.storyId));

  return (
    <Sheet open={open} onOpenChange={(v) => !v && onClose()}>
      <SheetContent className="w-96 overflow-auto">
        <SheetHeader>
          <SheetTitle className="text-sm">{card.title}</SheetTitle>
        </SheetHeader>

        <div className="mt-4 space-y-4">
          {/* Status & Meta */}
          <div className="flex flex-wrap gap-2">
            <Badge variant="outline" className={`${colors.bg} ${colors.text} ${colors.border}`}>
              {card.status.replace(/_/g, " ")}
            </Badge>
            {card.complexity && (
              <Badge variant="outline" className={COMPLEXITY_COLORS[card.complexity] || ""}>
                {card.complexity}
              </Badge>
            )}
            {card.businessValue && <Badge variant="outline">{card.businessValue}</Badge>}
          </div>

          <Separator />

          {/* Details */}
          <dl className="grid grid-cols-2 gap-2 text-xs">
            <dt className="text-muted-foreground">Story ID</dt>
            <dd className="font-mono">{card.storyId}</dd>
            <dt className="text-muted-foreground">Module</dt>
            <dd className="font-mono">{card.moduleId}</dd>
            <dt className="text-muted-foreground">Layer</dt>
            <dd>Layer {card.layer}</dd>
            <dt className="text-muted-foreground">Slice</dt>
            <dd>Slice {card.slice}</dd>
            <dt className="text-muted-foreground">Attempts</dt>
            <dd>{card.attemptCount}</dd>
            {card.completedAt && (
              <>
                <dt className="text-muted-foreground">Completed</dt>
                <dd>{card.completedAt}</dd>
              </>
            )}
          </dl>

          {card.prUrl && (
            <>
              <Separator />
              <div>
                <p className="text-muted-foreground mb-1 text-xs font-medium">Pull Request</p>
                <a
                  href={card.prUrl}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-xs text-blue-400 underline"
                >
                  {card.prUrl}
                </a>
              </div>
            </>
          )}

          {dependsOn.length > 0 && (
            <>
              <Separator />
              <div>
                <p className="text-muted-foreground mb-1 text-xs font-medium">
                  Depends On ({dependsOn.length})
                </p>
                <div className="space-y-1">
                  {dependsOn.map((dep) => {
                    const depColors = getStatusColor(dep.status);
                    return (
                      <div key={dep.storyId} className="flex items-center gap-2 text-xs">
                        <span className={`h-1.5 w-1.5 rounded-full ${depColors.dot}`} />
                        <span className="font-mono text-[10px]">{dep.storyId}</span>
                        <span className="text-muted-foreground truncate">{dep.title}</span>
                      </div>
                    );
                  })}
                </div>
              </div>
            </>
          )}

          {blockedBy.length > 0 && (
            <>
              <Separator />
              <div>
                <p className="text-muted-foreground mb-1 text-xs font-medium">
                  Blocks ({blockedBy.length})
                </p>
                <div className="space-y-1">
                  {blockedBy.map((dep) => {
                    const depColors = getStatusColor(dep.status);
                    return (
                      <div key={dep.storyId} className="flex items-center gap-2 text-xs">
                        <span className={`h-1.5 w-1.5 rounded-full ${depColors.dot}`} />
                        <span className="font-mono text-[10px]">{dep.storyId}</span>
                        <span className="text-muted-foreground truncate">{dep.title}</span>
                      </div>
                    );
                  })}
                </div>
              </div>
            </>
          )}
        </div>
      </SheetContent>
    </Sheet>
  );
}
