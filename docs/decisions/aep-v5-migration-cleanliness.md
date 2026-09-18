# AEP v5：把專案 migrate 乾淨，以及沒有用過 AEP 的專案

狀態：提案（2026-09-18，使用者要求 review 三個 downstream 專案的 migration 之後整理）。
相關：[preview 採用與雙版本並存](aep-v5-preview-adoption.md)、[v4 sunset](aep-v4-sunset.md)、[原始專案遷移紀錄](../audits/2026-09-10-aep-v5-live-migrations.md)、[layer／wave 封裝](aep-v5-layer-wave-encapsulation.md)

## 使用者的定義

一旦用 CLI，就是全新的設計。migrate 的目標不是「v4 與 v5 並存」，而是 skill 加 CLI 引導 agent 把專案遷乾淨。專案分兩種：用過 AEP（v4）的，和沒用過的；兩種都應該以專案自己的設計紀錄與 Git 紀錄為依據完成遷移。

## 三個 downstream 的現況（2026-09-18 觀察）

三個專案都在 2026-09-10 遷移，之後全部工作都寫在 native stores。機械層面是乾淨的：

| 項目 | MITS | looplia | Rewarc-AutoResearch |
| --- | --- | --- | --- |
| `aep migrate plan` already_migrated／`migrate verify` | 是／通過 | 是／通過 | 是／通過 |
| 遷移後改動 legacy stores 的 commit | 0 | 0 | 0 |
| 遷移後改動 native stores 的 commit | ledger 24、roadmap 10、lessons 4 | 21、7、6 | 12、7、6 |
| import 記錄 | migration 與 openspec-import 各一筆，含 digest | 同 | 同 |

但以「乾淨」的標準看，三個專案都停在同一個狀態：

| 殘留 | MITS | looplia | Rewarc |
| --- | --- | --- | --- |
| `openspec/` | 917 檔、11 MB | 1,388 檔、9.2 MB | 670 檔、4.2 MB |
| 已 commit 的 v4 skill（`.agents/skills`＋`.claude/skills`，各 48 個 `aep-*` 目錄） | 676 檔、5.5 MB | 489 檔、4.3 MB | 796 檔、6.0 MB |
| `product-context.yaml`、`product/`、`project-convention/`、`lessons-learned/`、`skills-lock.json` | 都在 | 都在 | 都在 |
| AGENTS.md | 20 行 v4／v5 route 區塊，2026-09-10 的文字 | 同 | 同 |
| story `imported` 未 reconcile | 96／96 | 345／345 | 79／79 |
| 匯入的 decision 仍 pending | 32／32 | 53／53 | 18／18 |
| layer 沒有 description | 20／21 | 50／51 | 33／34 |
| wave pending | 79 | 192 | 79 |
| lessons：複製成檔案 vs 成為 lesson 記錄 | 99 檔／4 筆 | 185 檔／6 筆 | 68 檔／6 筆 |

三個結構性後果：

1. **v4 skill 還在被 host 載入。** Claude Code 讀 `.claude/skills/`，Codex 讀 `.agents/skills/`，所以每個 agent 仍看得到 48 個 `/aep-*` v4 skill。AGENTS.md 的 route 區塊（「Retained v4 instructions apply only to v4 work」）就是為了壓住它們而存在。殘留不清，route 就拿不掉；使用者在 `aep init` 上不想要的那段文字，根源在這裡。
2. **匯入的記錄是惰性的。** 所有 imported story 沒有一筆經 `aep story reconcile` 用 Git 證據確認；匯入的 decision 全部 pending，沒有接受也沒有 supersede；容器沒有 outcome。它們佔了記錄數的大半（MITS 548 筆中 imported story 96、layer 21、wave 79、decision 32），但 readiness 與 context 都把它們當未驗證，等於只是搬進來的檔案。
3. **legacy lessons 沒有進記錄圖。** 遷移把 `lessons-learned/` 的檔案複製到 `lesson-learned/`，但只有少數成為 lesson 記錄；`aep context` 只沿記錄的 `refs` 走，所以這些檔案只剩 `aep lesson find` 能搜到。

## 沒有用過 AEP 的專案

`aep migrate plan` 在沒有 `product-context.yaml` 的 repo 直接回 `No legacy source product-context.yaml`（exit 2）。`aep openspec import` 只認 OpenSpec 目錄。`aep --skill project --ref context-sources` 講的是「一個任務要湊齊哪些來源」，不是「從 README、docs/design、ADR、CHANGELOG 與 git log 建出初始 roadmap、decision、story」。所以第二種專案目前只有 `aep init` 加一份兩句話的 AGENTS.md，之後全靠 agent 自己判斷。

## 提案

### A. migrate 增加「清理」階段（用過 AEP 的專案）

- 指引（`migrate/SKILL.md` 加一段）：`migrate verify` 通過並 commit 後，移除 host 會載入的 v4 skill（`.agents/skills/aep-*`、`.claude/skills/aep-*`、`skills-lock.json`），移除或歸檔 `openspec/`、`product-context.yaml`、`product/`、`project-convention/`、`lessons-learned/`；Git 歷史與 import 記錄的 digest 就是來源證據，不需要在工作樹保留副本。清理後把 AGENTS.md 的 route 區塊換成兩句話的入口。
- CLI 候選：`aep migrate cleanup --dry-run` 列出會移除的路徑，並核對每個路徑都出現在 import 記錄的 `consumer_source_hashes`／`source_digest` 裡；`--apply` 執行並寫一筆事件（用 `--note`）。不列在 import 記錄裡的檔案不動，報告出來。
- 使用者決定：清理是刪除（靠 Git 找回）還是搬到單一 `legacy/` 目錄。提案傾向刪除，因為 sunset 決策已經說 v4 不再維護。

### B. 匯入記錄的處置（用過 AEP 的專案）

- 指引：遷移後對 imported story 做三選一：有 Git 證據就 `story reconcile`；沒有但已不做就標 `superseded`；仍要做的改成 pending 並補 change。匯入的 decision 要嘛接受（有來源）要嘛 supersede；空描述的 layer 補 outcome 或移除，沒有成員的 wave 移除。
- CLI 候選：`aep check` 對「imported 超過 N 天且未 reconcile」與「decision pending 且 `legacy_*`」給 warning；`aep status` 把 imported 與 live 分開計數。

### C. 沒有用過 AEP 的專案：從歷史建立脈絡

使用者補充（2026-09-18）：偵測到是 AEP 專案時，記錄本身就是脈絡來源，不需要再翻 Git；只有其他專案要導入時，才透過 `.git` 的歷史來梳理專案結構。

- 指引：新增 `migrate --ref bootstrap`（或放在 `project`）：讀 README、docs/design（或專案自己的設計目錄）、ADR 目錄、CHANGELOG、最近的 git log 與 open PR；用 roadmap 記錄寫方向，用 decision 記錄寫已成立的選擇（`data.source` 帶 path 與 commit），用 story 記錄寫進行中與明確待辦，來源不足的留成 draft。每筆記錄的 `refs` 或 `data.source` 都要指回原始檔案與 commit，讓 `aep context` 能沿線找到。
- CLI 候選：`aep migrate plan --source <dir>` 目前只接受 legacy 結構；可加 `--from-docs <dir>` 產生「候選記錄」草稿，交由 agent 分類，不自動接受。

### D. lessons 進記錄圖

- 遷移時把每個複製的 lesson 檔建成 lesson 記錄，`refs` 指向它談到的 story（能對到的話），沒有對象的只留檔案。這樣 context 反向邊才會把它們帶出來。

## 驗證方式

- 對三個 downstream 各做一次 A 與 B（使用者授權後），觀察：清理後 `aep check` 通過、`aep --skill` 只列 native 與專案 skill、host 不再列 `/aep-*`；imported 記錄數歸零或全部有處置狀態；AGENTS.md 只剩入口兩句。
- 對一個沒用過 AEP 的真實專案做 C，觀察 agent 是否能只靠指引與 CLI 建出可用的 roadmap／decision／story，且每筆都有來源。

這份文件只記錄觀察與提案；沒有修改 downstream 專案。
