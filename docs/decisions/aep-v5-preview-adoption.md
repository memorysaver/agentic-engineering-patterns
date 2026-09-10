# AEP v5 preview：雙版本並存與保留來源的 migration

日期：2026-09-10。狀態：**方向已接受；實作與發布證據另行記錄**。

依據本次使用者決定：先記錄設計、用 subagents 分別完整 review，再收斂 `feat/aep-5-native-cli`，準備給 downstream 實際測試。第一個候選採 `5.0.0-preview.1`；它不是 stable 5.0，也不宣稱已有 production pilot 證據。

## 已接受的決策

1. 保留 v4.1 的安裝 skills、原始資料與既有發布。v5 是可明確選用的 preview，使用獨立 Rust binary 與內嵌 native guidance。
2. 使用者明確指定的 workflow 優先，其次為專案宣告的 default。既有 v4 專案不因安裝 binary 或閱讀 catalog 自動切換。一次任務／handoff 要帶上版本、來源、store 與 worktree；缺能力須明示，不能偷偷 fallback 到另一版。
3. 兩套指引可並存。工作執行選定一套資料 owner；v5 接手後，v4 檔案成為 migration 時點的來源快照，不做双寫或背景同步。
4. Migration 保留 v4 source bytes，建立新 v5 資料與 source commit/path/digest 關係。完成後可以直接在舊專案觀察新結構；試跑也可以放在隔離 checkout。保留來源不等於支援任意切回 v4 繼續最新工作。
5. 新 stores 採 `project-roadmap/`、`project-ledger/`、`project-rules/`、`lesson-learned/` 與 `.aep/config.toml`。保留 `product-context.yaml`、`product/`、OpenSpec、`project-convention/`、`lessons-learned/` 及舊設定。自訂 store 與來源重疊時阻止 conversion，不能用覆寫完成隔離。
6. Native 核心依 [context 與 self verification 決策](aep-v5-context-and-self-verification.md)：保留 8 個入口，通用 pattern 交模型／host；保留明示專案約束與可核對證據。Preview 首先交付可實跑的指引、來源保留、版本路由與政策一致性。

## 入口與切換

`AGENTS.md` 是短的選版入口。v4 選擇 installed `aep-*` skills；v5 選擇 `aep skills` 與 `aep skills show <name>`。CLI catalog 的 release/digest 說明實際讀到哪份 native guidance；專案 default 說明目前用哪套 workflow，兩者不是同一欄位。

初始化保留既有專案 instructions、legacy 安裝與非 AEP 設定。新增的版本路由需把 workflow 選擇與既有通用專案約束分清楚，不能把舊 AEP workflow 原文無條件重新啟用。Migration 保存原 entrypoint 作來源，產生以 v5 為 owner 的入口；保留可讀的專案約束，明示歷史 commands 的身份。

讀取 skill 或安裝 binary 不等於授權 migration。Migration plan 是可審查的 writes/diagnostics；apply 是明確切換。來源漂移需重新 plan，不能以重跑覆寫 native 已編輯資料。已執行 v5 新工作的回退另作交接／Git 恢復，不提供虛假的雙向 schema 相容。

## Preview 的驗收

- 原本 v4 default 的專案初始化／讀 catalog 後仍可選 v4；v5 手動選用與 migration 後的入口清楚且不互相矛盾。
- 在帶 v4 context、rules、lessons、skills 與指令入口的 disposable Git 專案跑 plan/apply/verify；原資料與 skill bytes 保留，v5 records 可查詢。
- 保留必要來源 snapshot、相對引用與 custom constraint mappings。缺失或衝突明示；歷史完成不變成 passing evidence。
- 首次 apply、重跑、source drift、target collision、store overlap 有行為驗證。新紀錄持續寫 v5，舊檔修改不會默默同步 native facts。
- 新 native 專案可在沒有固定 evaluator topology 下執行 checks 與完成允許的交付；專案明示 independent-review 要求仍有效。
- Native guidance 涵蓋 context 建構、project-owned verification 與 prototype 決策；既有 24 項都有保留／自主／外置的落點。Packaging 與實際 agent 行為觀察分開報告。
- 安裝後 binary 在 source checkout 之外提供全部 guidance，native archive 有 binary/license/checksum。版本、config、docs、tag 規則一致。
- Linux/macOS 最終候選 CI、獨立 review 與已執行檢查有具體記錄。缺少的 downstream 或 provider 實測不以 mock 成功代替。

## 發布與試用

v4.1 stable 保持不變；v5 preview 的 tag 名稱採 `v5.0.0-preview.1`。發布工作流要將含 prerelease suffix 的版本標為 GitHub prerelease，且不把它當最新 stable。

本次任務先完成可 review 的分支、候選 binary/archive、試用說明與檢查。公開 tag/release 的實際操作與證據另行記錄；準備完畢不等於已發布。Downstream 專案未選定前，不改動任何既有 consumer。

Downstream pilot 的核心觀察：能否從新結構找到 intent、rules、dependencies、verification 路徑與 lessons；能否以實際 evidence 收尾；是否錯用 v4 指引／來源；原型是否回答設計問題。Pilot 不是要求恢復所有 legacy patterns。

## 關聯決策與後續範圍

- [CLI-first](aep-v5-cli-first.md) 繼續有效，dashboard 暫緩。
- [Context 與 self verification](aep-v5-context-and-self-verification.md) 的分類與自主原則本次接受；具體資料欄位只在實作並驗證後宣稱可用。
- [舊盤點](../audits/2026-09-10-aep-v5-skill-migration-inventory.md) 是時點記錄，其通用 scheduler、backend、ingestion engine 待辦不構成 preview 前置條件。
- 任意外部互動 artifact 的完整 machine-enforced coverage、所有 host/provider、production migration、prototype 相對優勢仍需後續實驗；preview 保留清楚的證據界線。

## Downstream 實驗選擇

使用者先選定 Looplia，再明確加入自己的 MITS（Rust）與 Rewarc。三者同步使用獨立 local clone 觀察升級，保留原專案與未提交工作。實驗分別記錄 context 是否完整、既有 policy／consumer 是否仍有效，以及 self verification 的實際結果。Apply、驗證通過與恢復產品工作是不同階段；不以放寬規則換取 migration 成功。結果與修正收錄於 [三專案實驗報告](../audits/2026-09-10-aep-v5-downstream-pilots.md)。
