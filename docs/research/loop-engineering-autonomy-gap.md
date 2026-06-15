# Loop Engineering 研究 × AEP 自主開發差距分析

> **狀態：** 研究 + 比對成果（非實作計畫）。記錄「loop engineering 業界文章」與「AEP 現況」的對照，標出離完全自主開發 (fully autonomous development) 還缺的元素。
> **日期：** 2026-06-15　**分支：** `research/loop-engineering-autonomy-gap`

---

## 0. TL;DR

AEP 已經是一個**成熟的外層迴圈 (outer loop)** — 在 loop engineering 的「目標定義、工具環境、誠實驗證 (gen/eval)、host-agnostic 執行、制度記憶」這幾項上甚至**領先**業界文章。缺口集中在三處：

1. **內層迴圈的 context 紀律** — 單一 workspace agent 跑完 Phase 0–13，有 context rot 風險，缺 Ralph 式 fresh-context-per-task。
2. **自主復原能力** — 卡關時只會「重試同一招 → 升級給人」，缺「換策略」的階梯。
3. **合併後的生產回饋迴圈未閉環** — 缺自動 rollback / telemetry 驅動的 reflect / 自我餵食的工作發掘。這也是讓無人值守自主**安全**的前提。

---

## 1. 研究：Loop Engineering 是什麼

**定義（MindStudio）：** 「設計不只回應一次、而是 act → observe → decide → repeat 直到目標真正達成的 AI 系統」。各家 coding agent（Claude Code、Devin、Codex）品質差異「通常不是底層模型，而是 loop 設計」。

### 五大構件

1. **Clear Goal / Task Definition** — 定義「done」、可評估、可拆成可測子任務。「沒有 termination condition，agent 不是跑不停就是亂停。」
2. **Tool Set for Environment Interaction** — code execution / fs / shell / docs lookup / test runner。「agent 不能跑自己的 code，這 loop 只是在猜。」
3. **Context Management** — 摘要前次迭代、結構化 action log、剪裁無關 context。否則撞 token 上限或失憶。
4. **Termination Logic** — success / failure / escalation 條件。「沒有明確終止邏輯，loop 變資源黑洞。」
5. **Error Handling & Recovery** — 區分可復原 vs 阻斷、依錯誤型態**調整策略**。「同錯誤後重試同動作不是學習，是在空轉。」

### 基礎 pattern：ReAct（Reason + Act）

理解目標 → 嘗試動作 → 跑並觀察 → 推理 → 修正再試 → 重複至完成。

### 常見迴圈型態

Retry Loop / Plan-Execute-Verify / Explore-Narrow / Human-in-the-Loop。

### 實作 checklist（6 條）

先定 termination → 給結構化回饋（非 raw output）→ running log 週期摘要進 working memory → 每迭代設嚴格 tool-call 預算 → 測失敗路徑 → 用「真正無解」的任務驗證 exit 條件能觸發。

### Anti-patterns

無 exit / 策略停滯 (strategy stagnation) / context 溢出 / 目標模糊 / 缺工具存取。

### Ralph Loop（Geoffrey Huntley）關鍵原則

- **每迭代重新 malloc 整個 context window**，刻意「浪費」以避免 **context rot / compaction**（過 60–70% 進「Dumb Zone」）。
- **single-task per iteration** — 不做多階段規劃，降低失敗域。
- **context 隔離** — 不同任務用獨立 context，避免規格汙染。
- **Architectural back-pressure** — pre-commit hooks、property-based tests、自動部署、CDC、限制寫權限、audit log。
- **Safety engineering 讓激進自主變安全** — Huntley 的 agent 全 sudo 直推 master，靠的是完整測試 + < 30s rollback 的保護冗餘。
- 核心：**verification precedes autonomy**、**engineering trumps coding**。

---

## 2. 對照記分卡

| Loop-engineering 元素                                             | AEP 現況                                                                                                   | 評級                                                                | 證據                                   |
| ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- | -------------------------------------- |
| ① Clear Goal / Task Definition                                    | `product-context.yaml`、stories、OpenSpec specs、`contracts.md`、`dispatch_score`/`readiness_score`        | ✅ 強                                                               | `dispatch/SKILL.md`、`build` Phase 1–3 |
| ② Tool Set for Environment                                        | workspace agent 全工具、dev server、`ports.env`、test runners                                              | ✅ 強                                                               | `build` Phase 4/6、`init.sh`           |
| ③ Context Management（外層）                                      | CHECK 委派 Haiku/`codex exec`（避免 orchestrator 膨脹）、`lessons.md` 注入（cap 2000 tokens）              | ✅ 強                                                               | `tick-protocol.md` CHECK→ACT           |
| ③ Context Management（內層）                                      | **單一 workspace agent 跑完 Phase 0–13**，task-by-task 但**同一 context**；`init.sh` 只在 reset 後被動復原 | ⚠️ 缺口 G1                                                          | `build/SKILL.md` 全 13 phase           |
| ④ Termination Logic                                               | goal driver layer 邊界自停、`--max-turns 200`、`layer_complete` 條件、escalation triggers                  | ✅ 大致強（但「unsolvable」其實只是「打到上限」）                   | `tick-protocol.md` ⑦                   |
| ⑤ Error Handling & Recovery                                       | gen/eval loop、stuck detection + liveness、orphan 再領養、retry 計數                                       | ⚠️ 部分（缺「換策略」）                                             | `tick-protocol.md` ④⑤                  |
| Ralph：single-task fresh context                                  | 一個 task 一個 commit，但 context 不重置                                                                   | ⚠️ 缺口 G1                                                          | —                                      |
| Ralph：architectural back-pressure                                | 有 CI / gen-eval / contracts                                                                               | ⚠️ 部分（缺 post-merge 自動 rollback / property tests / audit log） | 缺口 G4                                |
| verification precedes autonomy                                    | gen/eval 分離、`feature-verification.json`「只有 evaluator 能改」                                          | ✅ 強                                                               | `gen-eval/SKILL.md`                    |
| per-iteration budget / 結構化回饋 / 測失敗路徑                    | 有 cost 追蹤、signals；codex 有 `token_budget`                                                             | ⚠️ 部分（預算非全 backend 硬約束）                                  | 缺口 G7                                |
| 「discovers → assigns → verifies → persists → hands off」自我餵食 | assigns/verifies/persists 有；**discovers 靠人工** envision/reflect                                        | ⚠️ 缺口 G6                                                          | `reflect` Step 1 手動問人              |

---

## 3. 缺口分類

### Bucket 1：執行/戰術層缺口（可自動化）

- **G1 — 內層 fresh-context per task**（防 context rot；Ralph malloc）。現況 lead 一路跑完 13 phase。
- **G2 — 換策略復原階梯**（genuine adaptation）。現況 eval FAIL 同一 generator 同思路再修，5 輪打滿升級人；缺「重讀 spec → 換做法 → 拆 story → 換 agent → 才找人」。
- **G3 — 設計歧義 / 視覺品質自主評斷**。`auto_design` 只是自動跑互動式 `/aep-design`；視覺品質明文「agent 無法判斷」靠 `.5` polish layer 人工。
- **G4 — Post-merge guard & 自動 rollback ⭐ 安全關鍵**。現況 merge 後即 wrap，無生產健康監控 / 自動 revert / canary / audit log。
- **G5 — Telemetry 驅動 reflect / outcome 評估**。現況 `reflect` 逐一問人，outcome contract 明文 pause 等人工判斷。
- **G6 — 自我餵食工作發掘 ("discovers")**。新工作只能從人工 envision/reflect 進入。
- **G7 — Loop hygiene**。per-phase token/tool-call 硬預算、termination 區分「打到上限」vs「真正無解」。

### Bucket 2：技術上可自動化、但**建議保留人工**（待拍板）

| 關卡                        | 可自動化途徑                            | 建議                               | 理由                         |
| --------------------------- | --------------------------------------- | ---------------------------------- | ---------------------------- |
| S1 產品願景 `/aep-envision` | telemetry + 市場訊號生成 hypothesis     | **保留人工**                       | 高風險、定義「做什麼」是戰略 |
| S2 架構決策 `/aep-map`      | gen/eval 架構提案                       | **人工核准**（agent 提案、人按鈕） | 難回滾、跨層影響             |
| S3 Outcome contract 判定    | 量化指標可自動 (見 G5)；質化保留        | **混合**                           | —                            |
| S4 成本/優先序權衡          | `dispatch_score` 已自動；殘留為預算上限 | **policy 化**（預算寫成 config）   | —                            |

### Bucket 3：本質保留人工

最終問責 / 生產事故價值判斷、倫理與商業風險決策。

---

## 4. 建議優先序（供後續實作規劃）

| 優先   | 缺口                                   | 為何                                             |
| ------ | -------------------------------------- | ------------------------------------------------ |
| **P0** | G4 post-merge guard、G2 復原階梯       | 沒 G4 → 無人值守不安全；沒 G2 → 一直 spin 回找人 |
| **P1** | G1 fresh-context、G5 telemetry reflect | 內層品質/規模 + 閉合外層回饋迴圈                 |
| **P2** | G6 自我餵食、G3 視覺自主、G7 hygiene   | 推向真正連續自主                                 |

> 逐檔案的實作藍圖（新增 reference / config flag / 修改點）已草擬，待決定推進哪些缺口後再展開為正式 spec。複用既有抽象：`executor.spawn/nudge/check`、gen/eval 協定、signals、`product-context.yaml` config、autopilot state schema。

---

## Sources

- [What Is Loop Engineering? — MindStudio](https://www.mindstudio.ai/blog/what-is-loop-engineering-ai-coding-agents)
- [What Is Agentic Coding? — MindStudio](https://www.mindstudio.ai/blog/what-is-agentic-coding)
- [Mastering Ralph loops (Geoffrey Huntley) — LinearB](https://linearb.io/blog/ralph-loop-agentic-engineering-geoffrey-huntley)
- [The Ralph Wiggum Loop — codecentric](https://www.codecentric.de/en/knowledge-hub/blog/the-ralph-wiggum-loop-autonomous-code-generation-with-a-fresh-context)
- [Loop Engineering — Cobus Greyling](https://cobusgreyling.medium.com/loop-engineering-62926dd6991c)
- [snarktank/ralph](https://github.com/snarktank/ralph) · [vercel-labs/ralph-loop-agent](https://github.com/vercel-labs/ralph-loop-agent)
