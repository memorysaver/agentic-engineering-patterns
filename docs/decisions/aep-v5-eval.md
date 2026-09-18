# AEP v5：`aep eval`，中立的工程品質監看

狀態：已接受並實作第一版（2026-09-18，使用者提出；原名 `aep watch`，改名 `aep eval`）。四個開放決策使用者都選建議方案：`~/.aep` 同時是安裝腳本的家目錄、`eval` 是第九個內建 skill、`watch` 在 Herdr 下預設 spawn、第一版包含 snapshot／report／skill／watch。
相關：[migration 乾淨度](aep-v5-migration-cleanliness.md)、[layer／wave 封裝](aep-v5-layer-wave-encapsulation.md)、[結構優先的判斷原則](../lessons/2026-09-17-project-a-layer-20-design-rounds.md)

## 目的

把這幾天人工做的「downstream 觀察」變成 CLI 的正式功能。`aep eval` 是一個中立的監看者：評價目前專案有沒有照 AEP 的流程走、有哪些軟體工程與 DevOps 實務可以改善。它不評價專案的產品決策、功能取捨或技術選型；只維護工程品質。

觀察到的需求來源：
- 觀察者的判斷應以結構證據為準（記錄、連結、readiness、check、Git），不以模型的細節反應為準。
- 證據要離開專案 repo 存放，避免污染專案的 ledger，也避免把私有專案的內容寫進公開文件。
- 觀察者不介入：不提示被觀察的 agent、不代答問題、不改專案檔案、不提交它的工作。

## 三層設計

| 層 | 責任 | 形式 |
| --- | --- | --- |
| 機器層設定 | 整台電腦的 AEP 家目錄與 eval 紀錄 | `~/.aep/config.toml`、`~/.aep/eval/` |
| CLI（Rust，唯讀、決定性） | 蒐集結構事實、套用規則產生報告、保存觀察記錄 | `aep eval snapshot` / `report` / `record` / `list` / `show` / `watch` |
| skill `eval`（觀察者程序） | 在 Herdr 之下讓一個專用 agent 依程序監看、交叉核對、寫觀察 | `aep --skill eval` 與其 references |

CLI 負責「能算的」，skill 負責「要看的」。報告裡每條發現都指向一個結構事實或一份保存下來的證據。

## `~/.aep`

```
~/.aep/
  config.toml            機器層設定
  eval/
    <project-id>/        由 git remote（host/owner/repo）或路徑 digest 決定
      <run-id>/
        manifest.json    專案路徑、remote、head、CLI build sha、模式、起訖時間
        snapshot.json    結構事實（下表）
        report.md/json   規則產生的發現，附實務類別、嚴重度、證據指標
        observations/    觀察者 agent 寫入的紀錄：transcript 摘錄、宣稱與證據的對照
        copies/          當時的檔案副本
        SHA256SUMS
```

`config.toml` 第一版欄位：

```toml
schema_version = 1

[eval]
root = "~/.aep/eval"
observer_kind = "codex"      # Herdr 啟動觀察者 agent 用的 kind
retention_days = 90

[eval.projects."github.com/owner/repo"]
observer_kind = "claude"      # 選用的專案覆寫
```

安裝腳本的 `AEP_HOME` 預設也改到 `~/.aep`（`builds/` 搬過去），`~/.local/bin/aep` 連結不變。現有 `~/.local/share/aep/trials/` 的證據包搬到 `~/.aep/eval/_manual/`，只搬不改。

## `aep eval snapshot`：CLI 蒐集的結構事實

全部唯讀，對專案不寫任何檔案。

| 群組 | 事實 |
| --- | --- |
| 記錄 | `aep check` 結果與 warnings；各 kind／status 數量；imported story 未 reconcile 數；legacy 標記的 pending decision 數；沒有 description 的 layer／wave 數；事件總數與帶 note 的比例（近 30 天另計）；accepted change 無 published spec；integrated story 無 delivery receipt |
| 驗證 | `config.checks` 數量、`required_checks`、`independent_review`；stale evidence 數；review 依 `attribution_class` 分布；blocking review |
| 交付 | delivery receipts、release 記錄、closed change 數 |
| Git | 分支、dirty 檔數、未推送 commit 數與最舊未推送的天數、attempt 與 worktree 的對應（孤兒 worktree、running 超過 N 天）、protected paths |
| Legacy | v4 skill 目錄、AGENTS.md 是否還有 route、legacy stores 是否存在、migration receipt 與 verify 結果 |
| DevOps | CI workflow 檔案是否存在、是否執行了 `config.checks` 裡的命令（字串比對）、secret scan 工具是否設定、lockfile、CHANGELOG、release tag |

## `aep eval report`：第一版規則

每條規則有 id、實務類別、嚴重度（info／advice／warn）、條件與訊息。第一版：

| id | 類別 | 條件 |
| --- | --- | --- |
| VER-001 | 驗證 | `config.checks` 為空：verify 只能空過 |
| VER-002 | 驗證 | `independent_review = true` 但近期 review 全是 `host_reported` |
| VER-003 | 驗證 | stale evidence 存在且其 story 仍 in_progress |
| REC-001 | 記錄 | imported story 未 reconcile 超過 N 天 |
| REC-002 | 記錄 | 近 30 天事件帶 note 比例低於 50% |
| REC-003 | 記錄 | layer／wave 沒有 description |
| REC-004 | 記錄 | 接受 change 的 `accepted_by` 沒有引用使用者陳述（只能 info） |
| DEL-001 | 交付 | closed change 沒有 published spec |
| GIT-001 | Git | 未推送 commit 超過 3 天 |
| GIT-002 | Git | snapshot 時工作樹有未提交檔案 |
| GIT-003 | Git | running attempt 超過 N 天或 worktree 沒有對應 attempt |
| LEG-001 | Legacy | 仍有 v4 skill 目錄或 AGENTS.md route 但已有 migration receipt |
| OPS-001 | DevOps | 沒有 CI workflow |
| OPS-002 | DevOps | CI 沒有執行 `config.checks` 的命令 |
| OPS-003 | DevOps | 沒有 secret scan 設定 |

規則只描述結構與流程；沒有任何規則評價 story 的內容、優先序或技術選擇。

## `aep eval watch`：Herdr 之下的監看模式

- 需要 `HERDR_ENV=1`；不在 Herdr 之下時只提示改用 `snapshot`／`report`。
- 建立一個 run，用 `aep --skill eval` 產生觀察者 prompt（含 run id、專案路徑、要觀察的 pane），在被觀察的 pane 旁邊（同一個 workspace）split 一個 pane，以 `config.eval.observer_kind` 啟動專用觀察者 agent 並送出 prompt。`--no-spawn` 只印出 Herdr 指令。
- 觀察者 agent 依 skill `eval` 的程序：用 Herdr 找到同一 cwd 的工作 agent；在它每次 idle／done 時做 `aep eval snapshot --run <id>`、讀 transcript、對照記錄與 Git；把宣稱與證據的差異用 `aep eval record --run <id> --file <obs.json> --note` 寫進 run；結束時 `aep eval report --run <id>`。
- 中立規則寫在 skill 裡：不提示工作 agent、不回答它的問題、不改專案檔案、不提交它的工作、不評價產品決策。
- 從 Codex 或 Claude Code 啟動：使用者在自己的 session 說「開始 aep eval」，工作 agent 執行 `aep eval watch`。

## 對專案的寫入

預設完全不寫。只有使用者明確要求時，`aep eval export --lesson --run <id>` 才把一份摘要以 lesson 記錄寫進專案，refs 指向相關 story，內容只含結構性發現。

## 開放決策

1. `~/.aep` 是否同時成為安裝腳本的 `AEP_HOME`（`builds/` 搬過去）。建議是，讓機器層只有一個位置。
2. `eval` 做成第九個內建 skill，還是 `reflect` 的一個 reference。建議第九個，因為讀者是觀察者 agent 而不是工作 agent；preview proof 與 catalog 測試的「8」要一併改。
3. `watch` 預設自動 spawn 觀察者，還是只印指令。建議在 `HERDR_ENV=1` 時預設 spawn，`--no-spawn` 關掉。
4. 第一版規則的門檻（N 天、比例）先寫死在 CLI，之後再開放到 `config.toml`。

## 第一次實跑的修正（2026-09-19）

第一次在真實專案上跑 `watch` 發現兩個問題，都已修：

1. 觀察者 pane 開錯地方。原本 split 的是執行 `watch` 的 pane，觀察者落在觀察者自己的 workspace；使用者預期它出現在被觀察 agent 旁邊。改成 `herdr pane split --pane <target>`，觀察者現在開在目標 pane 同一個 tab。已啟動的觀察者用 `herdr pane move` 搬過去。
2. Codex 的 sandbox 擋住 Herdr socket 與 `~/.aep`。觀察者每跑一個 `herdr` 指令或 `aep eval record` 都要人核准。改成以 codex 為 kind 時，`agent start` 帶 `-s workspace-write -c sandbox_workspace_write.writable_roots=[<AEP home>, <Herdr socket 目錄>]`。其他 kind 不加參數。

## 驗證方式

- Rust fixture：對一個小型專案 fixture 跑 `snapshot`／`report`，每條規則各有一個觸發與一個不觸發的案例；`snapshot` 對專案沒有任何寫入（快照前後 tree digest 相同）。
- 實機：對三個 downstream 專案各跑一次 `snapshot` 與 `report`，人工核對每條發現是否對應真實狀態，並和 2026-09-17／18 的人工觀察結果比對。
- watch：在 Herdr 下對一個正在工作的 agent 啟動觀察者，確認觀察者沒有向工作 agent 送出任何輸入，run 目錄有 snapshot、observations 與 report。
