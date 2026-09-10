# AEP v5 skill 能力移植盤點

盤點日期：2026-09-10。來源為 `feat/aep-5-native-cli`，HEAD `34e5b9a` 加上本機尚未提交的 CLI-first 調整。本文件記錄目前實作與建議的移植工作，**不是能力移植完成報告，也不是新的架構決策**。

後續方向更新（同日）：使用者指定以 context 完整度與 self verification 為核心，gen-eval 等通用 pattern 交模型自主處理。[重新分類提案](../decisions/aep-v5-context-and-self-verification.md) 已逐項收斂 L01–L24，納入 verification-with-prototype，並指出 Rust review 政策的落差。下文保留原盤點與當時建議；「全部承接」及工作包順序不再能直接視為新的實作範圍。

## 結論與範圍

舊版 marketplace 收錄 **24 個 skills**；目前 Rust binary 內嵌 **8 個 native skills、2 份 references、2 份 entrypoint templates**。8 個 skills 與 2 份 references 的 CLI 輸出均與 native 原始檔逐字一致，但這只證明打包完整，不能證明舊版能力完整承接。

以下 24 項全部列入盤點：**15 項以補寫指引為主，9 項需要先重設 v5 契約，再移植指引**。這個分類表示後續工作的主要性質；不表示 24 項全部未實作，也不表示有 15 項已完成。尚無足夠的 v5 行為觀察，可以把任何一列標成「舊版全部責任已驗證承接」。

移植遵循已確認的範圍：

- Rust CLI 獨立執行；AEP 本身不依賴 Node、Bun、Web App 或執行中的 LLM server。專案的測試工具與 host agent 另由使用環境提供。
- 一次性 migration 完成後，以 v5 的 ledger、roadmap、rules、lessons 與 config 為準；不保留讀寫舊資料的日常執行路徑。
- [Dashboard 實作與封裝暫緩](../decisions/aep-v5-cli-first.md)。下游產品的 UI 設計方法、人的品質判斷，以及文字狀態摘要，仍是需要盤點的 AEP 能力。
- 移植單位是能力與必要資源，不要求保留 24 個舊名稱、slash commands、plugin 安裝方式或 shell scripts。先沿用 8 個 native 入口，再依實際選用觀察決定是否拆出新入口。

## 盤點依據

| 證據 | 能證明什麼 | 不能證明什麼 |
| --- | --- | --- |
| [Legacy marketplace](../../.claude-plugin/marketplace.json) 與下方各列的 `SKILL.md` | v4.1 對外目錄為 24 項；各 skill 宣告的用途、流程與交接責任 | 舊流程在所有 host 都實際成功 |
| [Native build](../../crates/aep-cli/build.rs)、[guidance loader](../../crates/aep-cli/src/guidance.rs)、`aep skills --json` | binary 讀取 `skills/native/`；內嵌內容、版本、摘要與引用可離線取得 | 目錄的 `complete: true` 不代表 legacy 功能覆蓋率 |
| [CLI command 定義](../../crates/aep-cli/src/cli.rs)、[record operations](../../crates/aep-cli/src/commands.rs)、[workflow](../../crates/aep-cli/src/workflow.rs)、[core](../../crates/aep-core/src/lib.rs) | 目前真正存在的操作、readiness、證據與狀態限制 | 通用 `Record.data` 能保存資訊，不代表已有專用語意檢查 |
| [CLI scope audit](2026-09-09-aep-v5-cli-focus.md)、[lifecycle fixtures](../../crates/aep-cli/tests/lifecycle.rs) | 已記錄 39 個 Rust 測試與獨立 binary 驗證；基本生命週期有具體實作 | code review 的「command families 齊全」不等於 24 個 skills 行為齊全 |
| [Legacy behavior observations](../../evals/skill-behavior-parity.json) | 可作為移植情境來源，目前有 23 筆案例 | 不是 v5 證據；`run.review_scopes` 還寫 22 contracts，案例也缺 `easy-explain`，不能直接沿用其 pass 結論 |
| [初次 v5 audit](2026-09-09-aep-v5-implementation.md) | 有限情境下的 native skill 選用與操作觀察 | 未涵蓋全部 legacy 能力與資源 |

本次逐項比較目錄、各 skill 的主要流程、相關 reference/template 與 native 實作。未執行舊 host scripts、外部 telemetry 來源或 24 項完整 agent 演練；後續完成條件列在本文末段。

## Native 8 個入口目前承接到哪裡

| Native skill | 已有內容 | 主要尚缺 |
| --- | --- | --- |
| [project](../../skills/native/project/SKILL.md) | 最小初始化、保留既有 instructions、rules 分類、check config、local procedures | 完整導入說明、stack/setup 範例、E2E skill 產製與升級 |
| [roadmap](../../skills/native/roadmap/SKILL.md) | intent 與執行狀態分離、journeys、容器、ADRs、明確 dependencies/gates | 機會評估、產品 framing、story slicing、架構與 object model 方法 |
| [design](../../skills/native/design/SKILL.md) | change、BDD、決策與 acceptance；附 `bdd`、`records` 兩份 reference | exploration/技術設計方法、產品判準與人的校準流程 |
| [implement](../../skills/native/implement/SKILL.md) | readiness、claim/worktree、host handoff、attempt 狀態與恢復提醒 | 選工與 context 組裝、host 契約、專案 setup、持續執行與介入 |
| [validate](../../skills/native/validate/SKILL.md) | verify、獨立 review、兩輪上限、current evidence、gate | 非程式 artifact 的評估方法、評分範例、E2E 覆蓋與 failure 分類 |
| [deliver](../../skills/native/deliver/SKILL.md) | PR/merge/reconcile、spec publication、release gates | worker/process/worktree 收尾契約、完整 lessons 保存與 post-merge 工作 |
| [reflect](../../skills/native/reflect/SKILL.md) | lesson、rule proposal、反例與 held-out 驗證、adoption | 產品回饋分類、outcome 假說評估、重新切分 roadmap、上游整理流程 |
| [migrate](../../skills/native/migrate/SKILL.md) | 舊資料選擇、plan/apply/verify、來源保留與 OpenSpec mapping | 本身是 v5 新入口；資料搬移成功仍不能證明其內容會被其他 native skills 正確使用 |

## 24 項移植清單

「補寫」表示主要補方法、情境與 references，沿用已有 v5 操作；「重設」表示來源、狀態、證據或 host 邊界需要先定義，不能只換命令名稱。這是主要工作分類，補寫過程若發現缺少可驗證的底層操作，仍需另列實作工作。

優先序為建議：**P0** 補齊可驗證的核心工作循環；**P1** 產品判斷、狀態解釋與持續編排；**P2** 通用方法擴充。P1/P2 仍列在移植待辦，沒有因優先序而視為刪除。

### Product context：8 項

| ID／舊 skill | v5 現況 | 需要移植／建議落點 | 工作 |
| --- | --- | --- | --- |
| L01 [envision](../../skills/product-context/envision/SKILL.md) | `roadmap`/`decision` 可保存產品意圖，尚未提供 opportunity framing 流程 | 保留 proceed/kill/defer、persona/JTBD、MVP 邊界、活動 backbone、反方壓力測試；補到 `roadmap` references，結果寫 v5 roadmap/decision | 補寫 · P1 |
| L02 [map](../../skills/product-context/map/SKILL.md) | story/layer/wave、依賴與 gate 已存在；沒有系統性拆解方法 | 保留 system map、介面/失敗邊界、walking skeleton、integration stories、可驗收切片與 outcome 假說；補 `roadmap`/`design` references，以明確依賴取代數字 layer/slice 語意 | 補寫 · P1 |
| L03 [model](../../skills/product-context/model/SKILL.md) | 無 OOUX/ORCA 指引、專用 object-map 核准與失效檢查 | 保留 objects、relationships、CTAs、attributes、screen coverage；先定義 v5 設計文件、decision/change 引用與 draft/approved/stale 的證據契約，再放入 `design`/`roadmap` | 重設 · P1 |
| L04 [dispatch](../../skills/product-context/dispatch/SKILL.md) | `dispatch plan/start` 已判定 readiness、容量、scope overlap 與 claim；沒有業務排序 | 保留價值、解鎖效果、風險與 ambiguity 的比較方法，以及設計/實作 handoff 和 context 選材；由 agent 選工，Rust 保有 readiness/claim，補 `implement` reference | 補寫 · P0 |
| L05 [validate](../../skills/product-context/validate/SKILL.md) | native `validate` 與 verify/review/gate 已承接 implementation 證據主線 | 補產品/文件/spec 評估、vision alignment、walking-skeleton/INVEST 檢查、context 組裝與 findings 去重；保留 review-only 只回報的 v5 範圍，不照搬舊版自動改 artifact 流程 | 補寫 · P0 |
| L06 [calibrate](../../skills/product-context/calibrate/SKILL.md) | 無七維度判準、brief/capture 或 extension 流程；通用 decision 不等於校準已生效 | 移植 visual-design、ux-flow、api-surface、data-model、scope-direction、copy-tone、performance-quality；先定義依據、人的決定、適用範圍與失效條件，用 v5 設計/決策/契約承接，不恢復 `.5` 隱含關卡 | 重設 · P1 |
| L07 [reflect](../../skills/product-context/reflect/SKILL.md) | native 已有經驗與規則改善循環，但尚未完整承接產品學習循環 | 補 bug/refinement/discovery/opportunity shift/process 分類、outcome 評估、cost/retry 觀察與 re-slicing；分流到 story、roadmap/decision、rule。移植時清除現有 native prose 的 `monet-*` 專案名稱殘留 | 補寫 · P0 |
| L08 [watch](../../skills/product-context/watch/SKILL.md) | 沒有來源抓取、cursor、stable external identity、去重或監控 driver；`story new` 只承接最終寫入 | 先定義 observation、source identity、重試/去重與 cursor 提交契約，串接 `reflect`；保留 regression 與意圖變更的分類。driver 留在 host，新增入口或 reference 待情境驗證 | 重設 · P1 |

### Project setup：3 項

| ID／舊 skill | v5 現況 | 需要移植／建議落點 | 工作 |
| --- | --- | --- | --- |
| L09 [onboard](../../skills/project-setup/onboard/SKILL.md) | `init`/`doctor`、entrypoint 與 config 已存在；README 有原生安裝說明 | 補進 embedded `project`：首次使用說明、新專案/既有專案/單一工作入口、host/tool 能力檢查與安裝驗證。移除 plugin pin、per-host skill copies、舊 hooks 安裝依賴 | 補寫 · P0 |
| L10 [scaffold](../../skills/project-setup/scaffold/SKILL.md) | `project` 要求保留 stack、只補缺項，但沒有具體 setup/repair 方法 | 補既有專案盤點與冪等修復、workspace deps/env/port/seed 的專案契約、檢查設定範例；stack-specific generator 作選用 recipe，不能讓 Better-T-Stack/Node 變成 AEP 必需品 | 補寫 · P0 |
| L11 [e2e-skill-scaffolding](../../skills/project-setup/e2e-skill-scaffolding/SKILL.md) | 可配置 checks/environment，能解析 BDD；尚無完整 project-owned E2E scaffold | 重設 policy、journey、tool selection、seed/preflight 模板的 v5 落點；明確分開 scripted tests、agent journey、API drivers、執行環境與證據。保留既有手寫 journeys，接 `project`/`validate` | 重設 · P0 |

### Development workflow：5 項

| ID／舊 skill | v5 現況 | 需要移植／建議落點 | 工作 |
| --- | --- | --- | --- |
| L12 [design](../../skills/agentic-development-workflow/design/SKILL.md) | native change/BDD/decision/accept 已有；OpenSpec 是明確 adapter | 補 exploration、替代方案、技術設計觸發條件與 design review 情境；使用 v5 change contract，移除依賴 `/opsx:*` 與安裝套件的基本流程 | 補寫 · P0 |
| L13 [launch](../../skills/agentic-development-workflow/launch/SKILL.md) | Rust 驗證 base、建立 claim/worktree 並回傳 launch request；明示 `worker_started: false` | 定義 host-confirmed worker handle、cwd/store 綁定、setup 結果、啟動失敗與 orphan re-adoption；補 bootstrap/context 範例。claim 建立不可當作 worker 已運作 | 重設 · P0 |
| L14 [build](../../skills/agentic-development-workflow/build/SKILL.md) | 分散承接於 `implement` → `validate` → `deliver` | 補可恢復的任務進展、專案 setup、以實際 diff 選回歸/E2E、decision 中斷與返回；完成條件以 v5 attempt/check/review/delivery 表達，不重建舊 phase 0–13、progress JSON 與 signals 狀態機 | 補寫 · P0 |
| L15 [wrap](../../skills/agentic-development-workflow/wrap/SKILL.md) | delivery、spec publish/close、attempt done 已存在；沒有 worktree remove/worker teardown 操作契約 | 定義先保存 lessons/必要執行證據，再確認 integration、停止所擁有的 worker/process、清理可刪 worktree 的順序與恢復方式；保留未合併或未保存工作。不能把 `attempt.status = done` 當成清理完成 | 重設 · P0 |
| L16 [git-ref](../../skills/agentic-development-workflow/git-ref/SKILL.md) | Rust 已保存明確 base/branch/worktree，delivery 有明確 PR base 與 integration 檢查 | 整理成 `implement`/`deliver` Git reference：如何選與確認 base、合理 commit、rebase/conflict/reflog 與 cleanup；不沿用 `develop → main` 自動猜測、固定 `feat/*` 命名或重複 shell 實作 | 補寫 · P0 |

### Human alignment：1 項

| ID／舊 skill | v5 現況 | 需要移植／建議落點 | 工作 |
| --- | --- | --- | --- |
| L17 [human-alignment](../../skills/human-alignment/SKILL.md) | `status/query/context/timeline` 提供 records、readiness 與 events；沒有完整 attention set、等待人的問題或 drift 摘要 | 先定義從 v5 records/receipts 推導的文字 pulse：目前位置、變動、阻塞、誰需要決定什麼、證據範圍。未知日期保留未知；事件記錄時間不代替發生時間。HTML brief/fact-binding renderer 與 dashboard 顯示層暫緩 | 重設 · P1；HTML 暫緩 |

### Patterns：7 項

| ID／舊 skill | v5 現況 | 需要移植／建議落點 | 工作 |
| --- | --- | --- | --- |
| L18 [gen-eval](../../skills/patterns/gen-eval/SKILL.md) | native 已有獨立 review、兩輪上限、實際 diff 風險分級與 freshness | 補通用 artifact 評分維度、良莠範例、generator/evaluator context 邊界、findings 合併與 recovery/failure 分類；共用 `validate` references，不另建第二種 verification receipt | 補寫 · P0 |
| L19 [executor](../../skills/patterns/executor/SKILL.md) | Rust 管 worktree/attempt；host 負責真正 spawn，尚無完整可用性與 liveness 操作指引 | 定義 capability detection、spawn/steer/liveness/gate/stop/resume 的 host 契約與回報；區分 session-bound 與 process-bound worker。只為選定 host 提供已驗證 recipes，不自動恢復 tmux/cmux fallback | 重設 · P1 |
| L20 [autopilot](../../skills/patterns/autopilot/SKILL.md) | 有可組合的 dispatch/attempt/review/deliver 操作，沒有完整持續執行 loop | 重設基於 v5 狀態的 bounded tick、重複執行、停止/恢復、stuck vs 等待人、收尾/選工排序與授權邊界；post-merge 觀察與 driver 分開。不可把舊 JSON 狀態機或 Node tick scripts 搬回核心 | 重設 · P1 |
| L21 [workflow](../../skills/patterns/workflow/SKILL.md) | 無專用指引；host 可提供 delegation，但不等於已有 workflow 方法 | 保留何時值得拆解、fan-out/synthesis、adversarial verification、競爭候選與有限重試的方法；先作可選編排 reference，去除特定 Workflow tool/keyword 的必需性 | 補寫 · P2 |
| L22 [workflow-feedback](../../skills/patterns/workflow-feedback/SKILL.md) | native `reflect` 已指出 local rule/procedure/upstream 目的地 | 補 capture/review 兩模式、跨專案證據歸納、process/tech-stack/discovery/local 分類、重複項與 upstream 提案格式；接 `reflect`，寫檔與對外發送分開依工作授權處理 | 補寫 · P1 |
| L23 [design-lens](../../skills/patterns/design-lens/SKILL.md) | 尚未內嵌方法與 theory catalog | 保留 task/data 導向選 lens、quick/deep review、usability/accessibility、Nielsen severity 與證據引用；可放 `design`/`validate` reference。CLI 輸出也有適用情境，不能因 dashboard 暫緩整項刪除 | 補寫 · P2 |
| L24 [easy-explain](../../skills/patterns/easy-explain/SKILL.md) | 無對應入口，也沒有顯式要求重新解釋的流程 | 移植補足前提、使用專案詞彙、連續重述仍易懂的方法；詞彙來源改成 v5 roadmap/rules/glossary。若新增入口，需處理 explicit-only 語意；現有 loader 僅取 name/description，不證明 host invocation policy 生效 | 補寫 · P2 |

## 不宜只靠補文字的契約缺口

以下是上表「重設」項目的實際邊界，不表示本次已批准新增命令或 schema。

1. **設計／人的校準證據（L03、L06、L17）。** `Record.data` 可保存任意資料，但目前 `readiness` 不會因 story 引用了某個 object-map 或 decision，就檢查它是否已由人核准、是否過期。`gate evaluate` 目前圍繞明確 story scope、check/review receipts 與 environment；不能直接把建置前的品質偏好或資訊架構決定當成現有測試 gate。需選定如何把 decision revision 綁進 change/check，或擴充明確的核准契約，再加入 stale/draft 阻擋案例。
2. **執行環境與人工 journey（L11）。** 宣告 `checks` 可執行命令，但不會自動產生專案的 journeys、seed/preflight 或證明 agent 已操作 UI/CLI。需定義 acceptance criterion → 實際執行 → revision/environment/evidence 的追蹤；local 測試通過不能代替 deployed journey。
3. **Worker 生命週期與清理（L13、L15、L19）。** `dispatch start` 建立的是 prepared attempt，`worktree` 目前只有 inspect；`finalize_delivery` 設定 done 並保留 worktree。需明確分工：Rust 擁有可檢查的 claim/record/worktree 操作，host 擁有 agent handle 與生命週期；專案 setup process 也要有 ownership，不能照搬用 port 任意 kill 的舊 recipe。
4. **監控與持續編排（L08、L20）。** durable observation identity、cursor、driver handle、tick/action 恢復尚無完整 v5 契約。單次 transactional record write 不等於整輪跨來源抓取與 host 操作可重試。先完成可手動呼叫、可重入的有限一輪，再接 host scheduler；不需要為此把 Node 或排程 daemon 打進 binary。
5. **文字狀態摘要（L17）。** 現有查詢足以取得許多事實，但「等誰決定多久」「計畫落後實作」「某能力是否可用」仍需明確來源與推導規則。先輸出附 record/revision 的文字摘要；不以新 UI 作為完成條件。

## References、templates、scripts 的移植方式

只移植 `SKILL.md` 的摘要會遺失方法；把整個舊資料夾內嵌則會帶回錯誤的執行契約。以下依資源責任整理，生成的副本不重複計算成能力。

| 資源來源 | 應保留的內容 | v5 處理方式 |
| --- | --- | --- |
| [Product shared templates](../../skills/product-context/_shared/templates/)：opportunity brief、context document、system map、story spec、technical spec、agent topology | 問題框定、介面/失敗邊界、可驗收 story 與 handoff 問題 | 改寫成 roadmap/design references 或可讀範例；`product-context-schema.yaml` 與 object-map YAML schema 只供來源語意對照，另定 v5 契約 |
| [Map references](../../skills/product-context/map/references/)、[ORCA](../../skills/product-context/model/references/orca-process.md)、[calibration references](../../skills/product-context/calibrate/references/) | walking skeleton、技術規格觸發、object model、七維度 brief/capture 與差異擴充 | 保留方法；重寫 `.5` layer、`calibration.history`、`product/maps` 的讀寫與核准規則 |
| [Dispatch references](../../skills/product-context/dispatch/references/) | 業務優先序比較、context 選材、校準/設計相關內容 handoff | 供 agent 判斷；移除 duplicate readiness 計算、YAML lock 與固定 token 配額假設 |
| [Gen/eval references](../../skills/patterns/gen-eval/references/) | rubrics、示例、獨立性、failure taxonomy、recovery、verification 成本觀察 | 合併進 validate references；欄位、風險下限與 receipts 對齊 Rust，不重建 `verification-recipe.json` 消費鏈 |
| [E2E resources](../../skills/project-setup/e2e-skill-scaffolding/)、[workspace setup](../../skills/project-setup/scaffold/references/workspace-hook.md) | policy、journey/tool 分離、三種驗證面向、冪等 seed、環境 preflight | 產製 project-owned procedure/checks；處理 cli、web、API、無執行介面的情境，不強制 web server；已有專案檔案保留 |
| [Executor resources](../../skills/patterns/executor/)、[autopilot protocol](../../skills/patterns/autopilot/references/tick-protocol.md)、[wrap convergence](../../skills/agentic-development-workflow/wrap/references/convergence.md) | host capability、liveness、有限執行、恢復、證據保存與停止順序 | 重寫為 v5 host 契約與操作範例；舊 backend scripts、signal schemas、autopilot state scripts 不直接內嵌 |
| [Telemetry ingestion](../../skills/product-context/_shared/references/telemetry-ingestion.md)、[attention set](../../skills/product-context/_shared/references/attention-set.md)、[drift facts](../../skills/product-context/_shared/references/drift-facts.md) | source identity、去重、回歸辨識、人的待辦、事實與推論界線 | 定義 v5 observation/summary 契約；來源失敗須明示，不以空資料假裝沒有問題 |
| [Design-lens resources](../../skills/patterns/design-lens/references/)、[workflow catalog](../../skills/patterns/workflow/references/pattern-catalog.md) | 通用分析與評估方法 | 可按需收進 references，保留出處；將 host-specific 呼叫換成能力描述 |
| [Legacy file resolution](../../skills/product-context/_shared/references/file-resolution.md)、[YAML guardrails](../../skills/product-context/_shared/references/yaml-guardrails.md)、[vocabulary](../../skills/product-context/_shared/references/aep-vocabulary.schema.json) | 需要 migration 理解的舊語意 | 舊格式解析留在明確 migration/adapters；日常指引使用 v5 schema、`aep check`、revision 與 transaction，不建立新舊雙寫或 fallback |
| [Human-alignment renderer/scripts](../../skills/human-alignment/scripts/) | 可參考事實來源與可追溯性；HTML 展示不是核心資料契約 | 文字摘要移植；Node derivation/assembly 與 HTML audit/render pipeline 暫緩，不納入 native runtime |

目前內建 references 只對外提供 Markdown，可包含巢狀路徑；local references loader 則只讀取單層、有效名稱的 `.md`。因此舊的 `briefs/` 移到 local skill 時需調整結構；JSON schemas 或 `.tmpl` 也不會因放入 bundle，就能透過目前 `aep skills show --ref` 取得並產製。模板要嘛轉成可讀 Markdown 範例，要嘛先定義資源輸出/產製介面。project-owned skills 使用 `project-rules/skills/` 或 config 的 `skill_paths`，不恢復各 host 複製整套技能的安裝流程。

舊共用資源以 `_shared/` 為來源、per-skill 副本由 build 生成，見 [skill corpus rules](../../project-rules/skills.md)。正式移植時讀原始方法並重寫 native 資源；本次不修改生成檔或 legacy corpus。

## 建議工作順序與驗收情境

以下是後續 work packages。每項列出需驗證的行為，**目前均未因本次盤點而宣告通過**。

| 順序 | 工作包／涵蓋項目 | 最小驗收情境 |
| --- | --- | --- |
| 1 | P0：導入與專案驗證，L09–L11 | 新 Git 專案、已有自訂 instructions 的 Rust CLI 專案各完成 setup；重跑沒有意外 diff。生成符合 CLI 的 journey/check，不要求 Node/web server；保留手寫 journey，分清 local 與 deployed 證據 |
| 2 | P0：設計、選工、啟動與實作，L04、L12–L14、L16 | 對多筆 ready work 說明選工理由；欠缺 acceptance 的工作先完成 design。檢查錯 base、host 啟動失敗、worker 跑錯 cwd、session 中斷恢復；不重複 claim、不改 main checkout、不把 prepared 回報為 running |
| 3 | P0：驗證與收尾，L05、L15、L18 | code 與非 code artifact 各有獨立評估；review-only 不擅自修補。證據在 edit 後失效、兩輪後未解 blocking 有出口。合併後先保存 lessons/evidence，再清理已核實可清理的資源；未合併工作保留 |
| 4 | P0/P1：回饋學習，L07、L22 | 同一組 bug、產品假說失效、流程缺陷分流到正確 v5 record；新回饋可重新切分 roadmap。rule proposal 有反例/held-out 檢查；跨專案 capture 有來源且不自動對外發布 |
| 5 | P1：產品定義與設計判斷，L01–L03、L06 | 記錄一次 proceed 與一次 defer/kill；walking skeleton 跨主要活動、infra story 可無 journey。UI-facing 才做 ORCA；校準七維度都可選用，至少實跑 light/heavy 與 extension。未核准或 stale 判準不被當成有效 acceptance |
| 6 | P1：文字狀態，L17 | 同時存在未整合、已整合未部署、等待決策、未知歷史時間的 records；摘要能指出依據、阻塞與單一具體待辦，不把 receipt 缺失說成完成，不需要啟動 Web App |
| 7 | P1：持續執行與來源監控，L08、L19、L20 | 同一 tick/來源重跑不重複建工；部分來源失敗不丟 cursor；已完成 issue 再發作可辨識。host 活著但無 diff 不誤判死亡；等待人不視為 stuck；停止後可恢復且不擴張原授權 |
| 8 | P2：通用方法，L21、L23、L24 | 小任務不因有 workflow 而強制拆 agent；需要獨立驗證時能界定輸入/輸出與上限。design-lens 找出具體可用性問題並引用觀察。連續兩次「再解釋」補足前提且保留專案詞彙 |

每個工作包完成時，更新本表中的來源能力、native 落點、刻意取消的舊行為與具體觀察。新入口名稱與是否拆 skill 應由可發現性情境決定，不能只追求數量從 8 變 24。

## 完成移植的共同條件

- 每個 L01–L24 都有明確去向：native procedure/reference、Rust 操作、project-owned procedure 或明確延後的展示功能。任何取消需說明能力影響，不能用「合併到某 skill」代替證據。
- 移植後的正常流程只使用 v5 狀態。舊 YAML、legacy signals、plugin 路徑只出現在 migration 或歷史說明中；不要求向下相容。
- binary 可在沒有 source checkout、legacy skills、Node/Bun 的環境提供新增 guidance/resources。實際 project checks 或 host 操作的外部需求須與 AEP 本體分清。
- packaging/frontmatter/links 檢查與行為觀察分開報告。Rust fixtures 驗證 deterministic constraints；agent 情境驗證選用、判斷、handoff 與失敗回復。既有 fixtures 可重用，新增測試針對新契約，不為每段 prose 建鏡像測試。
- 在安裝產物上重演代表性 v5 情境，記錄實際載入的 skills/references、record/revision、檢查結果與未涵蓋的 host/provider。不得將 legacy observation 的 pass 複製成 native pass。

## 本次已執行的檢查與剩餘工作

本次重新執行 `cargo run -q -p aep-cli -- skills --json`，核對 marketplace 的 24 個唯一來源路徑；逐項比較 8 個 native skills 和 2 份 references 的 CLI JSON `content` 與原始檔，共 10 項完全一致。盤點時的 bundle digest：

```text
fe2a87369de52e210f989bb81e2d1a0ecb448fc55c757568d56abc8018ff0655
```

文件檢查確認 L01–L24 與 marketplace 一一對應，沒有漏列、重複或額外項目；本文及同步更新的 README、plan、CLI scope audit 共 82 個本機文件連結均可解析。`git diff --check` 與新增文件的空白格式檢查通過。本次只改文件，未重跑 Rust 測試；上述 39 項測試結果引用前次 scope audit。

本次交付是移植盤點、資源處理方式與驗收清單；尚未移植任何新 skill、擴充 Rust 契約或恢復 dashboard。後續依本表實作與補上 native 行為證據，並繼續完成 [implementation plan](../plans/aep-v5-implementation.md) 所列 downstream pilot、最終 CI 與發佈工作。
