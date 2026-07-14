#!/usr/bin/env bash
# /aep-scaffold existing-project converge — the MECHANICAL, idempotent fixes.
# Applies: A canonical aep-* skills layout, C gitignore, E version-pin
# recommendation (recommend-only, never auto-run). Each step is a no-op when
# already satisfied. NEVER overwrites hand-authored content.
#
# The model-driven converge steps are NOT here (they need delegation/judgment):
#   B  e2e-test skill  → delegate to /aep-e2e-skill-scaffolding
#   C  git repo        → `git init -b main && git add -A && git commit -m "chore: initial commit"` (only if no .git)
#   C  OpenSpec / hook → follow /aep-scaffold Phase 5 and Phase 7
# See references/converge-flow.md. After running this, delegate B + fill the
# remaining C gaps, then re-run scripts/audit.sh until it exits 0.

echo "=== A. Canonical skills layout ==="
# Share one copy of each aep-* skill across runtimes. NEVER `rm` a real dir unless the canonical
# copy is known to exist — otherwise PROMOTE it (preserve content) before linking. A Claude-only
# install (real dirs in .claude/skills, none in .agents/skills) is the exact case this must not destroy.
if [ -d .claude/skills ] && [ -d .agents/skills ]; then
  ( cd .claude/skills
    for d in aep-*; do
      [ -e "$d" ] || continue                 # nothing to normalize
      [ -L "$d" ] && continue                  # already a symlink — leave it
      if [ -e "../../.agents/skills/$d" ]; then
        rm -rf "$d" && ln -s "../../.agents/skills/$d" "$d"          # canonical copy exists → safe to replace
      else
        mv "$d" "../../.agents/skills/$d" && ln -s "../../.agents/skills/$d" "$d"   # only copy → promote, don't destroy
      fi
    done )
fi
# CLAUDE.md = @AGENTS.md import — only create when ABSENT (never clobber a hand-authored CLAUDE.md).
if [ -f AGENTS.md ] && [ ! -f CLAUDE.md ]; then printf '@AGENTS.md\n' > CLAUDE.md; fi
# A real-content CLAUDE.md that isn't the import is flagged for manual merge — NOT overwritten.
if [ -f CLAUDE.md ] && [ "$(head -1 CLAUDE.md | tr -d '[:space:]')" != "@AGENTS.md" ]; then
  echo "NOTE: CLAUDE.md has hand-authored content — merge it into AGENTS.md by hand, then set CLAUDE.md to '@AGENTS.md'."
fi

echo "=== C. Gitignore (workflow dirs) ==="
grep -q '.dev-workflow/' .gitignore 2>/dev/null || printf '\n# Agentic development workflow\n.dev-workflow/\n' >> .gitignore
grep -q '.feature-workspaces/' .gitignore 2>/dev/null || printf '.feature-workspaces/\n' >> .gitignore

echo "=== E. Version pin (recommend-only — never auto-run) ==="
# Re-pinning AEP is a deliberate, own-PR action (README). If your local pin lags the latest release,
# run these yourself, in their own PR:
echo "  npx skills add memorysaver/agentic-engineering-patterns@<newtag> -a claude-code --skill '*' -y"
echo "  npx skills add memorysaver/agentic-engineering-patterns@<newtag> -a codex        --skill '*' -y"
echo "  then normalize .claude/skills/aep-* symlinks (step A), bump the AGENTS.md pin note, commit --no-verify"
