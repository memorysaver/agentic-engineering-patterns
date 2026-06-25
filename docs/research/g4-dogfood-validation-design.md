# G4 — Host-aware Dogfood Validation 預設設計

> **狀態：** 設計 spec（待展開為 skill 變更）。屬 [loop-engineering-autonomy-gap](./loop-engineering-autonomy-gap.md) §3 的 **G4** 子設計：部署後在 staging/production 上的驗證，依 host 採原生方法。
> **日期：** 2026-06-15　**分支：** `research/loop-engineering-autonomy-gap`

---

## Context

G4 是「合併後的生產回饋閉環」。現況：AEP 的 dogfood（`/aep-build` Phase 6）只在**本地 localhost**（`ports.env` 的 `BASE_URL`）跑，且**前提是 agent-browser 有裝**否則整個 phase skip；**完全沒有 staging/production 部署後驗證**。本設計補上兩件事：

1. **Dogfood 方法 host-aware** —— Claude Code 自動判斷是否用 agent-browser；Codex 採原生 browser / computer-use。
2. **部署後在 staging/production 驗證** —— 新增 post-deploy dogfood，目標 URL 來自 config 或 CI。

---

## 決策（已拍板）

| 項目                  | 決定                                                                                                                                      |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| staging/prod URL 來源 | **config 優先，fallback CI** —— `topology.routing.deploy_targets.{staging_url,production_url}`；缺則從 CI/deploy 輸出（如 preview URL）讀 |
| 接入點                | **新 G4 post-deploy 步驟 + 升級 Phase 6 為 host-aware**（兩者並存）                                                                       |
| 發現問題時            | **自動建 story 進 dispatch**（走 `/aep-reflect` 分類器，連動 G6）                                                                         |

---

## 研究依據（host 原生能力）

- **Claude Code：** agent-browser 是原生瀏覽器工具（CDP 驅動 Chrome、accessibility-tree `@eN` refs、screenshot `--annotate`、video、auth vault），`/agent-browser:dogfood` 已是 Phase 6 用的探索式測試流程。健康偵測 `agent_browser_healthy()`（`agent-browser navigate about:blank`）自含於 `patterns/executor/references/dogfood-validation.md`。
- **Codex：** computer-use（GPT-5.4 原生：截圖 + 滑鼠鍵盤 + 寫 Playwright）與 in-app browser（Atlas）**僅桌面 app**；`codex exec`（headless）**沒有**，只能寫並跑 Playwright 腳本或退回 agent-browser CLI。→ Codex 必須分桌面 / headless 兩條路。

來源：[OpenAI Codex app](https://developers.openai.com/codex/app)、[GPT-5.4 — OpenAI](https://openai.com/index/introducing-gpt-5-4/)、[Codex superapp — MacStories](https://www.macstories.net/news/openai-unveils-codex-superapp-update-with-computer-use-automations-built-in-browser-and-more/)、[Codex for Chrome — eigent.ai](https://www.eigent.ai/blog/codex-for-chrome)。

---

## 預設選擇邏輯（dogfood method 偵測）

延用 `executor.detect()` 的精神，新增一層方法偵測（host × mode）：

```
dogfood_method():
  resolve HOST + mode via executor.detect()

  if HOST == claude:                       # 任一 mode
    if agent_browser_healthy():  return "agent-browser"      # /agent-browser:dogfood
    else:                         return "degrade"           # 非 UI→API/curl；UI→human-eval

  if HOST == codex:
    if mode == codex-subagent and computer_use_enabled:      # 桌面 app
                                  return "codex-native"      # in-app browser + computer-use
    else:                                                    # codex-exec / headless
      if playwright_available():  return "playwright-script" # GPT-5.4 原生會寫
      elif agent_browser_healthy(): return "agent-browser"   # CLI 退路
      else:                       return "degrade"
```

| Host / mode                  | 預設原生方法                         | 偵測                        | 退路                             |
| ---------------------------- | ------------------------------------ | --------------------------- | -------------------------------- |
| Claude Code（任一 mode）     | `/agent-browser:dogfood`             | `agent_browser_healthy()`   | 非 UI→API/curl；UI→human-eval    |
| Codex 桌面（codex-subagent） | native in-app browser + computer-use | desktop + computer-use 啟用 | Playwright skill → agent-browser |
| Codex headless（codex-exec） | 寫並跑 Playwright 腳本               | playwright 可用/可裝        | agent-browser CLI → API 檢查     |

> 所有方法統一輸出同格式報告（`/agent-browser:dogfood` 的 severity/category/repro 模板），讓下游分類器 host-agnostic。

---

## 目標 URL 解析

```
target_url(env):                 # env ∈ {local, staging, production}
  if env == local:               # 現況不變
    source .dev-workflow/ports.env → return $BASE_URL
  else:
    u = product-context: topology.routing.deploy_targets.<env>_url
    if u: return u                                   # config 優先
    else: return <讀 CI/deploy 步驟輸出的 preview/deploy URL>   # fallback CI
```

---

## 接入點

### (1) 升級 Phase 6（本地，pre-merge）

`/aep-build` Phase 6 把「agent-browser 沒裝就 skip」改為呼叫 `dogfood_method()`：Claude→agent-browser、Codex→原生。`env=local`，URL 來自 `ports.env`。報告仍寫 `.dev-workflow/dogfood-<feature>.md`。

### (2) 新 G4 post-deploy 步驟（staging/prod，post-merge）

在 autopilot tick 的 wrap 後（或 `post-merge-guard`）新增：merge→（觸發/等待 deploy）→`target_url(staging|production)`→`dogfood_method()` 跑驗證→寫報告。維持 orchestrator boundary（讀 signals/報告 + 跑 gh/CLI，不讀 workspace code）。

---

## 發現問題時的行為

- **dogfood 發現的問題** → 餵 `/aep-reflect` 分類器 → 自動建 bug/refinement story 進 `product-context.yaml` → dispatch（連動 G6 自我餵食）。
- **硬性 regression（健康訊號）** → 另走 G4 post-merge guard 的 `auto_revert` 政策（預設保守：先告警、人工確認後才 revert）。兩條路分開：dogfood 找 UX/功能問題建 story；guard 找服務性 regression 決定回滾。

---

## config 新增（product-context.yaml）

```yaml
topology:
  routing:
    deploy_targets:
      staging_url: "https://staging.example.com" # 選填；缺則 fallback CI
      production_url: "https://example.com"
    dogfood:
      method: auto # auto | agent-browser | codex-native | playwright
      post_deploy_env: staging # staging | production | none
      on_issue: create_story # create_story | escalate
```

---

## 實作時會動到的檔案（待展開）

| 檔案                                                                       | 變更                                                                                                   |
| -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `agentic-development-workflow/build/SKILL.md`                              | Phase 6 改呼叫 `dogfood_method()`（host-aware），不再「沒裝就 skip」                                   |
| 新 `patterns/.../references/dogfood-validation.md`                         | `dogfood_method()` 偵測 + `target_url()` 解析 + 報告格式                                               |
| `patterns/executor/references/codex-native.md`                             | 新增 codex-subagent 用 in-app browser / computer-use 做 dogfood 的 recipe；codex-exec 用 Playwright    |
| `patterns/autopilot/references/tick-protocol.md` + `post-merge-guard.md`   | 新增 post-deploy dogfood 步驟                                                                          |
| `product-context/reflect/SKILL.md`                                         | 接收 dogfood 報告 → 分類 → 建 story（已有分類器，補來源）                                              |
| `patterns/executor/references/dogfood-validation.md`（health probes 自含） | `agent_browser_healthy()` / `playwright_available()` 等 probe 內含；後續泛化為 `e2e_tool(target_type)` |

---

## Verification（實作後）

1. **Claude Code**：裝/不裝 agent-browser 各跑一次 Phase 6 → 確認自動選 agent-browser / 正確 degrade。
2. **Codex 桌面**：codex-subagent 跑 post-deploy → 確認用 in-app browser + computer-use 驗證 staging URL。
3. **Codex headless**：codex-exec 跑 → 確認改寫並跑 Playwright 腳本（無 computer-use 時）。
4. **URL 解析**：設 `deploy_targets.staging_url` → 用之；移除 → 確認 fallback 從 CI 輸出取得。
5. **on_issue**：故意留一個 UX bug → 確認自動在 `product-context.yaml` 建出 bug story 並進 dispatch。
6. **boundary**：確認 post-deploy 步驟只讀報告/signals + 跑 CLI，不讀 workspace code。
