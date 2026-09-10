# AEP v5：以 context 與 self verification 收斂 skill 分類

日期：2026-09-10。狀態：**分類與核心方向已接受，實作證據另行記錄**。使用者已要求據此收斂 v5 分支，完成獨立 review 並準備 downstream preview；見 [preview 採用決策](aep-v5-preview-adoption.md)。

使用者已指定方向：AEP 的核心是 context 的完整度與 self verification 指引；gen-eval 等通用 pattern 交給模型自主運用。本文件記錄這個方向在既有 8 個 native skills、24 項 legacy 能力與 Rust CLI 上的落點。以下「目前」描述研究時點的實作；它與目標的差距需由後續 implementation audit 核實，接受決策不等於所有契約已實作或發布。

## 結論

**保留 `project / roadmap / design / implement / validate / deliver / reflect / migrate` 八個入口，改以 context 建構與可觀察驗證分配責任。** 入口描述使用者要完成的工作，references 提供按需方法，project-owned procedures 承載產品實際操作。模型自行選擇探索、拆工、review 與重試策略；Rust 保持 records、版本、依賴、隔離、明示 policy 與證據檢查的機械責任。

舊 24 項是來源能力清單，後續不以逐項恢復舊流程作為完成條件。核心能力要有去向；固定編排、評分與 host scripts 可以明確退出 native 範圍。`migrate` 是 v5 新增的轉換入口，不是第 25 個待移植 legacy skill。

不另增 `context`、`prototype`、`gen-eval` skill：`context` 已是資料查詢命令；prototype 是 `design` 解答未知問題的方法；gen-eval 是模型可選策略。若往後實際選用觀察顯示入口難找，再調整 catalog，現在沒有證據支持先擴張名稱。

## 研究依據與界線

| 來源 | 本次採用的發現 | 證據界線 |
| --- | --- | --- |
| [24 項盤點](../audits/2026-09-10-aep-v5-skill-migration-inventory.md)、[marketplace](../../.claude-plugin/marketplace.json)、legacy skills | 區分產品知識、可重用方法、操作契約與固定編排；不是把 24 個名稱塞回 8 個檔案 | 盤點中的 15 項補寫／9 項重設是此前建議，不能繼續當成全部必做 |
| [Native skills](../../skills/native/)、[context 實作](../../crates/aep-cli/src/commands.rs)、[verification 實作](../../crates/aep-cli/src/workflow.rs)、[config](../../crates/aep-core/src/lib.rs) | 現有命令可當骨架，但 context coverage、runtime proof 與 review autonomy 仍有缺口 | 下文清楚區分既有能力與新增建議 |
| idea：`deep-research/agentic-engineering-patterns/14-pstack-verification-with-prototype.md`、`lesson-learned/05-aep-next-gen-adversarial-edge-case-search.md` | 已有 AEP 下游「對抗過約束 → 低收益 edge case search」觀察；prototype 是可能改善方法 | 本機來源根目錄為 `/home/memorysaver/idea`；不把筆記當成本次重跑實驗 |
| [AEP verification economics](verification-economics.md)、[v4.1 behavioral rebaseline](fable-5-1-behavioral-rebaseline.md) | 舊制度已遭遇驗證成本、抽象情境膨脹與低影響 findings 的反覆循環 | 既有現場觀察支持重新設計，未證明所有 independent review 都應取消 |
| pstack 官方 [create-verification-skill](https://raw.githubusercontent.com/cursor/plugins/main/pstack/skills/create-verification-skill/SKILL.md) | 專案自己的操作工具、Feature Map，以及產生指引後真正跑一次的要求 | 本次閱讀原始指引，未安裝或執行 pstack |
| pstack 官方 [prototype playbook](https://github.com/cursor/plugins/blob/main/pstack/skills/poteto-mode/playbooks/prototype.md) | 原型用來回答具體設計問題，觀察對應 surface；產出決定與可拋棄原型 | 來源主張；沒有與 AEP 做同任務的效果比較 |
| pstack 官方 [maintain-verification-skill](https://raw.githubusercontent.com/cursor/plugins/main/pstack/skills/maintain-verification-skill/SKILL.md) | Feature Map 需以 source 與 live 行為核對；文件漂移和產品 regression 分開處理 | 取維護責任，不採用其固定平行 agent 分工 |

官方來源讀取日為 2026-09-10，連結指向當時的 `main`，未固定 commit。X Part 1／2 本次無法重新取回正文，文章層面的解讀來自已存在的 idea 筆記；本次直接核對的是上述三份 GitHub 指引。未驗證文章的產量數字，也未進行整個 plugin 稽核。

## 八個入口的責任

這是可按需選用的工作入口，沒有要求每個任務依序跑完八個 skill。

| CLI 指引入口 | 核心問題與責任 | 應留下的 context／證據 | 現有操作骨架 |
| --- | --- | --- | --- |
| `aep skills show project` | 這個專案怎麼理解、啟動、操作與驗證？建立／修復環境和專案驗證 procedure | rules index、實際 setup、verification procedure、Feature Map、可執行 checks | `init`、`doctor`、`config show/update`、`check` |
| `aep skills show roadmap` | 為誰解決什麼問題？下一個可觀察成果是什麼？ | 問題、persona/JTBD、outcome、journeys、邊界、system map、stories 與 dependencies | `roadmap`、`story`、`layer`、`wave`、`release`、`decision` |
| `aep skills show design` | 本次要改什麼？還缺哪個決定或實證？ | 小型 change contract、BDD、適用設計判準、prototype findings、decision 與排除理由 | `context`、`change`、`decision`、`spec check` |
| `aep skills show implement` | 在哪個基準、範圍與環境把工作完成？ | 任務 context、base/head、worktree/attempt、進度、未決問題與恢復資訊 | `context`、`dispatch plan/start`、`attempt`、`worktree inspect` |
| `aep skills show validate` | 哪些觀察足以支持完成主張？ | acceptance → 方法 → 結果／artifact；revision、environment、未涵蓋範圍 | `verify plan/run`、按政策使用 `review`、`gate evaluate` |
| `aep skills show deliver` | 這份已驗證工作實際交付到哪裡？ | PR／integration、spec publication、environment gate、保留的 lessons/evidence、資源收尾 | `deliver`、`spec publish/close`、`release promote` |
| `aep skills show reflect` | 新觀察改變了哪些已知事實或作法？ | bug／產品假說／規則修正的分流、lessons、verification map 維護、上游候選 | `lesson`、`reflect propose`、`rule adopt`，需要時回到 `story/roadmap` |
| `aep skills show migrate` | 哪些既有 context 必須帶到 v5？ | 來源版本、語意 mapping、尚未驗證的歷史主張、切換結果 | `migrate plan/apply/verify`、`openspec` adapter |

`status/query/context/timeline` 是跨入口的讀取面。人的狀態摘要在這些事實上組裝，不再需要獨立 human-alignment renderer。摘要能指出現在位置、證據、變化、阻塞與真正需要決定的事項即可。

## 24 項逐一去向

「保留」保留的是方法與必要 context，不保留舊固定流程；「自主」不再把該 pattern 當 native 必修程序。每列有一個主要責任歸屬，跨入口只交接結果，不複製整份方法。

| ID | Legacy skill | 主要落點 | 保留的能力與退出的責任 |
| --- | --- | --- | --- |
| L01 | `envision` | `roadmap` | 保留問題、機會、persona/JTBD、成功結果與 proceed/kill/defer 的依據；以任務需要決定分析深度 |
| L02 | `map` | `roadmap` | 保留 system map、journeys、walking skeleton、可驗收切片與介面依賴；移除數字 layer 的隱含流程 |
| L03 | `model` | `design` 按需 reference | 保留領域名詞、objects/relationships/actions 與畫面結構；ORCA 是工具，非所有 UI 工作的必經階段 |
| L04 | `dispatch` | `implement` | 保留選工依據、dependency/readiness 與 context handoff；排序由 agent 判斷，claim/容量/隔離由 CLI 維持 |
| L05 | `validate` | `validate` | 保留 code／文件／設計相對 intent 的驗證；以可追溯結果取代統一評分儀式，review-only 仍只報 findings |
| L06 | `calibrate` | `design` 按需 reference | 保留人的判準、examples/counterexamples、適用範圍與來源；七維度作選單，移除 `.5` gate 與 heavy 類型必定停下的安排 |
| L07 | `reflect` | `reflect` | 保留產品回饋、outcome 假說、bug/refinement/discovery 分流，形成 context 更新；清除 native `monet-*` 名稱殘留 |
| L08 | `watch` | `reflect` 接收觀察；driver 外置 | 保留 observation 的來源、時間、identity 與證據；來源抓取／cursor／去重由選定 adapter 負責，通用監控引擎退出 native 核心待辦 |
| L09 | `onboard` | `project` | 保留首次導入、既有 instructions 保護、能力探測；移除 plugin pin 與 per-host 技能複製流程 |
| L10 | `scaffold` | `project` | 保留實際 stack/setup、env/seed/isolation 與冪等修復；以既有工具為起點，不指定通用產品 stack |
| L11 | `e2e-skill-scaffolding` | `project` 產製；`validate` 使用 | 升級為 project-owned verification procedure + Feature Map；涵蓋 CLI/API/UI/library，保留已有 journeys，避免另建一份重複測試地圖 |
| L12 | `design` | `design` | 保留 intent、邊界、alternatives、acceptance；加入 verification-with-prototype，不把抽象 plan 的完整度當 gate |
| L13 | `launch` | `implement` | 保留 base/cwd/context/environment 的 handoff；當前 agent 可自己執行，只有選用 delegation 時才需要 host handle，不預設 spawn |
| L14 | `build` | `implement` | 保留成果、範圍、恢復資訊與實作中 self verification；固定 phase 0–13、重複狀態檔與 gen-eval 呼叫鏈退出 |
| L15 | `wrap` | `deliver` | 保留 integration 核實、spec、lessons/evidence 保存與所屬資源清理；done 不能代替 cleanup 證據 |
| L16 | `git-ref` | `implement` 的操作 reference | 保留 base/ref/worktree、衝突與恢復的必要指引，`deliver` 引用；不成為獨立 skill 或自動猜分支的腳本 |
| L17 | `human-alignment` | `roadmap` 的 status reference | 保留以 records/evidence 解釋目前狀態、缺口與待決問題；dashboard／HTML renderer 延後，不為文字摘要新增入口 |
| L18 | `gen-eval` | **模型自主**；證據原則歸 `validate` | 取消固定 generator/evaluator topology、分數尺與必跑 rounds；保留反證、真實觀察、findings impact，以及選用 reviewer 時的來源與 revision |
| L19 | `executor` | **host 能力**；交接邊界歸 `implement` | agent 選擇 host 已有能力；保留 worker/cwd/resource ownership 與恢復事實；通用 backend matrix、fallback scripts 退出核心 |
| L20 | `autopilot` | **模型／host 自主** | 持續執行與排程由 host 提供；AEP 保留可查詢進度、冪等操作、未解決問題與授權範圍，不重建 tick daemon／第二套狀態機 |
| L21 | `workflow` | **模型自主** | 拆工、fan-out、競爭方案與 synthesis 交模型選；不內嵌固定 topology catalog，也不因存在 skill 就要求多 agent |
| L22 | `workflow-feedback` | `reflect` | 保留有來源的 capture、共通模式與 upstream proposal；本地保存和對外發送依任務授權分開處理 |
| L23 | `design-lens` | `design` 按需 reference | 保留有來源的 usability/accessibility 判準，供 `validate` 使用；不固定跑完全部 theory families 或一律分數化 |
| L24 | `easy-explain` | **模型溝通能力**；專案詞彙屬 context | 不新增 native skill；保留 glossary 與缺失前提，依使用者要求重新解釋，不移植 explicit-only host 設定 |

patterns 目錄不能整包同樣處理：L18–L21 的通用編排退出；L22 的可累積經驗是 context 資產；L23 的領域判準可按需載入；L24 的溝通方式交模型，但 glossary 仍有保存價值。

## Context 完整度：足以做對與驗對

完整度不是讀完倉庫，也不是增加文件數量。判準是：**agent 是否能說明這次為何做、現況如何、受哪些約束、成功如何觀察，以及哪些資訊仍未知。**

| Context 面向 | 來源／需要知道什麼 | 缺口如何處理 |
| --- | --- | --- |
| Intent | 使用者請求、story、roadmap；outcome、範圍、授權到哪一步 | 補目前缺少的前提；會改變產品方向的歧義交人決定 |
| Reality | 目前 source、tests、runtime、diff、base/head；實際行為與歷史原因 | 讀 relevant code/history 或跑最小 probe，區分宣告與現況 |
| Constraints | 適用 rules、ADR、介面、人的設計判準及其版本 | 缺失／互相矛盾／已 superseded 的來源要可見，不能當有效 acceptance |
| Verification | 使用者路徑、操作工具、fixture、預期結果、local/deployed environment | 找出能驗的 surface；工具欠缺時先補最小 harness，無法執行則如實留下 gap |
| Continuity | dependencies、attempt/worktree、已做的檢查、未決問題、相關 lessons | 恢復既有工作與有效證據，避免重新開工或重複 claim |

建議以簡短的任務 context 摘要保存「主張 → 來源 → 版本／觀察時間 → unknown」，引用 canonical records 與 artifacts，按需讀原文。不規定固定 token 配額、全檔複製或獨立的 `context.json` 狀態機。當缺少來源會影響 acceptance 時，留下具體待查項；無關領域不強迫填表。

`aep context <id>` **目前**只走 record links，額外檔案需要 `--source`；其回傳也明示由 agent 選擇更多 context。它不會自動找出所有 relevant source、讀 runtime 或證明理解完整。先在指引補足來源選取與缺口報告，再用 pilot 判斷需要哪些 deterministic 檢查。

版本綁定需補一個真實缺口：verification fingerprint 已納入 linked decision/rule/lesson 等 record bytes，但不能據此宣稱 record 引用的任意外部文件、prototype 或 screenshot bytes 都已綁定。需要用於 gate 的依據必須有明確 artifact identity/digest 與失效規則；`Record.data` 可存欄位不等於已有 enforcement。

## Self verification：工具、路徑、可核對結果

本提案的 self verification 指 agent **親自取得可觀察證據，對照 acceptance，修正後重驗相關行為**。判斷可以由同一 agent 完成；工具輸出與實際副作用提供可核對依據。另一個 LLM 的評語是可選補充，無法代替執行。

責任分三層：

1. **AEP `project` 教如何建立驗證能力。** 找出已有啟動、操作與觀察工具，補缺項，真正執行產出的指引。
2. **Project-owned procedure 教如何驗這個產品。** 預計落在 `project-rules/skills/verify-<project>/`，或保留已有可發現的本地 procedure。可用 `aep skills show <local-name>` 讀取；具體 driver 仍是該專案的工具，不是 AEP 內建控制所有 app。
3. **AEP `validate` 教如何形成完成證據。** 根據 intent、實際 diff 與受影響路徑選 checks，說明結果能支持哪些主張，哪些尚未驗證。

建議的 project-owned procedure 包含：啟動與 preflight、操作方法、Feature Map、觀察與副作用、資料／環境隔離、所屬 process 清理、證據保存。產生時至少實跑一條代表路徑並確認清理後仍保有證據。這是受 pstack 啟發的 AEP 落點，沒有沿用 `.cursor/skills/` 或其固定 host 分工。[來源](https://raw.githubusercontent.com/cursor/plugins/main/pstack/skills/create-verification-skill/SKILL.md)

Feature Map 是「使用者功能 → 到達方式 → 操作方式 → 預期可觀察結果」的索引。roadmap/journey 記錄想要的產品體驗；Feature Map 記錄目前怎麼操作驗證，引用同一 acceptance，避免另存一份會漂移的需求。入口、介面或工具改變時，由交付／反思觸發受影響項目的實跑更新；產品 regression 不能靠修改地圖洗掉。[維護參考](https://raw.githubusercontent.com/cursor/plugins/main/pstack/skills/maintain-verification-skill/SKILL.md)

| 產品 surface | 可以構成 self verification 的例子 | 不能單獨支持的主張 |
| --- | --- | --- |
| CLI | 執行實際 binary、exit/stdout/stderr、前後檔案狀態；必要時 PTY | 編譯成功不證明 migration 或互動路徑正確 |
| API/service | 經公開 API 發 request、核對 response 和持久化／權限效果 | handler unit test 不證明部署 routing/auth 正確 |
| Web/desktop | 使用實際控制工具走路徑、觀察畫面與副作用 | 靜態 screenshot 不證明點擊／保存真的有效 |
| Library | 從公開 API 執行 consumer 範例或 compile/run harness | private helper 測試不證明使用者能整合 |
| 文件／research | 核對原始來源、連結、指令與關鍵主張 | 文筆流暢或模型評分不證明來源成立；無 runtime 的產物不強造 app journey |

每個重要完成主張應能追到 acceptance、實際 revision/environment、執行方法、預期與觀察結果、artifact 位置及限制。重用有效證據；變更後重驗受影響部分；只有新資訊或未解決問題才擴大驗證。無法取得 deployed environment 就保留該缺口，不用 local pass 代替。

目前 `verify run` 可執行 configured checks，記錄 command/result/environment/head/fingerprint，並偵測執行期間輸入漂移。**它目前依 process exit 等結果產生 receipt，沒有完整的 acceptance coverage 或外部 artifact ingestion 契約。** Scripted probe 可先接現有 checks；互動證據先保留可追溯 notes/artifacts，不能假稱已受 gate 強制檢查。後續應擴充既有 evidence 面，不另造第二套通用 receipt。

## 新 idea：verification with prototype

把它放在 **`design` 的按需 reference**，使用 `project` 提供的操作能力，沿用 `validate` 的證據原則。遇到實作方式、互動或性能的未知問題時，先把未知轉成可實驗的問題。

```text
任務與 context
  → 可觀察的問題／成功條件
  → 必要時做最小 prototype，操作並取得證據
  → 選定方案與記錄 decision/change
  → 正式 implement ↔ self verification
  → deliver → reflect／更新 context
```

Prototype 的完成條件是回答問題，並非完成 production code；採用官方 playbook 的「為具體決定做最小可觀察實驗」原則。簡單且方向已明的修改直接實作，不強制做多版本。可用序列探索；多 agent 與多模型只在有需要且授權允許時選用。[來源](https://github.com/cursor/plugins/blob/main/pstack/skills/poteto-mode/playbooks/prototype.md)

例如 AEP 遇到「單一大型 context 輸出是否容易漏掉設計判準」：選一個有 story、decision、rule 與 runtime 路徑的 fixture，比較整包輸出和來源索引加按需展開。讓一個冷啟動 agent 找到同一 acceptance 與驗證方法，記錄遺漏／錯誤引用、是否能跑到結果，再決定 API 形狀。這是待做實驗，不是已觀察成果。

原型先放隔離 scratch/worktree，保存問題、方法、觀察與限制；選定方案後把持久結論寫進 decision/change，正式候選另跑驗證。當前 `dispatch start` 是接受契約後的正式 attempt，不能為了建立原型就把尚未確定的 production contract 標成 accepted。第一階段可用一般 scratch；若確實需要跨 session 管理，再設計 research attempt，不先新增 prototype command。

人的主觀選擇以可看、可操作的候選與 tradeoff 支持；既有授權可決定的事項由 agent 完成。只有缺失的意圖、品質偏好或權限實際影響決定時才詢問，移除 legacy calibration 因維度分類而必定中斷的規則。原型會被正式重用時需達到正式實作的驗證標準。

## Gen-eval 退出後，哪些 Rust 政策要改

自主選擇方法與明示的專案約束需要一起成立。建議目標：**self verification 是一般預設；independent review 由專案 policy／任務的明確要求決定是否必需，agent 仍可主動選擇額外 review。** 選擇 reviewer 也遵循 host 的 delegation 授權。

| 現況 | 建議目標 | 實作前的界線 |
| --- | --- | --- |
| `Policy.independent_review` 預設 true，且 `risk != "light" || policy.independent_review` 令 standard/deep 即使設 false 仍必需 review | 新 native 預設不強制獨立 LLM；明示 project review policy 可要求指定 scope 的 review | 不是改 description 就生效；需改 risk/policy 判定、相關 gate/delivery/rule adoption 與 fixtures |
| `review request` 硬編碼兩輪，第二輪限定 blocking 修正 | 不以固定 rounds 決定任務何時完成；依實際未解決問題、取得新證據的價值與任務 budget 決定下一步 | 舊兩輪限制仍有效，直到 policy/receipt 契約一起修改；保留明確停止理由，避免無收益循環 |
| `validate` 主要敘述 review 流程 | 先寫如何操作、觀察、反證與判斷 coverage；review 是其中一種選用方法 | 同一 agent 的觀察不能被假記為另一個 independent reviewer |
| 已有 project config 或歷史 check/review receipts | 保留其來源和原始意思；政策變動後重新評估當前完成條件 | 不偷偷刪除專案明示 review 要求，不重寫歷史證據，不把缺席說成通過 |

保留 `review` 命令有實際價值：一些專案需要第二人 review；外部 reviewer 的 findings 和 revision 仍需保存。移除的是一律套用的模型編排與評分，不是移除 review 記錄能力。`verify/gate/deliver` 繼續核對所有明示 required checks、有效 evidence 與實際 integration；模型自治不改寫已接受的完成條件。

## 資源結構與實作順序

建議先新增少量 reference，每份有單一責任，其他 skill 只引用。以下路徑都是**預計新增**，目前不能當成已可用的 `--ref`：

| 擁有者 | 預計 references | 內容 |
| --- | --- | --- |
| `project` | `context-sources.md`、`verification-setup.md` | 專案來源與環境盤點；專案驗證 procedure 的產製／修復 |
| `roadmap` | `product-context.md`、`status.md` | 產品 framing、切片與依賴；有來源的狀態摘要 |
| `design` | `design-criteria.md`、`prototype.md` | model/calibrate/design-lens 的按需方法；實證探索；既有 `bdd`、`records` 留存 |
| `implement` | `handoff.md`、`git.md` | 任務 context、自己執行／委派、恢復及資源 ownership |
| `validate` | `self-verification.md` | acceptance coverage、工具與 runtime 證據、失敗／未知／失效的判斷 |
| `deliver` | `closure.md` | integration、證據保留與收尾；Git 操作引用 `implement` |
| `reflect` | `feedback.md` | observation 分流、更新 Feature Map/lessons/rules、上游候選 |
| `migrate` | 按既有 migration 缺口補充 | 舊資料語意與可追溯轉換；不恢復舊日常執行流程 |

Project-owned Feature Map 可先採單層 `references/feature-map.md` 與拆分檔；若專案已有巢狀 journeys，從 procedure 直接指向原檔。現有 local loader 的 `--ref` 只列單層合法 Markdown 名稱，不能假設 native 的巢狀 reference 行為也適用本地 procedure。大型地圖確有需要時才擴充 loader。

實作建議依依賴分四批：

1. **Context + verification 的第一個垂直切片。** 改 `project/validate` 指引，以 AEP 自身 CLI 建立可冷啟動使用的 verification procedure、Feature Map 與真實 runtime probe；已有 tests/checks 繼續重用。
2. **Autonomy 與 evidence 契約一致化。** 調整 review default、rounds、gate/delivery 等約束；定義必需 artifacts 的身份與 freshness。這批完成前，不宣稱一般 code story 已可只靠 self verification 交付。
3. **Design 的 prototype 路徑。** 補 `roadmap/design/implement` 的 context handoff 與判準、原型方法，以一個真實未定案問題跑完探索 → 決定 → 正式實作／重驗。
4. **交付、學習與導入收尾。** 補 `deliver/reflect/migrate`，驗證 lessons/evidence 留存、map 更新與外部 observation 分流。再做代表性下游 pilot、最終平台 CI 與授權發布。

通用 scheduler、跨 host executor、source ingestion engine、HTML dashboard 不再列在這一輪核心能力移植的前置條件；使用到哪個外部 adapter 才定義並驗證哪個邊界。這是相較上一份 inventory 的範圍收斂。

## 驗收：證明 agent 有能力閉環

| 情境 | 要觀察到什麼 |
| --- | --- |
| 冷啟動接手 | 只憑 catalog、task 和專案資料找到適用規則、當前行為與驗證入口；缺資料能指出具體來源，不靠先前對話記憶 |
| 已知功能的小修改 | 單一 agent 完成實作與有效 probe；不因 AEP 存在而強制 spawn、評分或多輪 review |
| 未定案設計 | 原型回答具體問題，決定引用觀察；沒有未驗證的抽象 edge-case 清單膨脹；正式版本另驗 |
| 假綠與版本漂移 | 編譯通過但實際行為錯誤的候選不能報完成；code／判準／必需 artifact 變更後相關證據失效 |
| 環境缺失 | 區分產品缺陷與 preflight 問題，報 blocked/未驗範圍，不用增加 LLM review 修補缺失環境 |
| 專案明示 review | 模型 autonomy 不繞過 project policy；缺必要獨立 review 仍不能 deliver |
| Feature Map 漂移 | 更新已變動操作，產品 regression 留作 defect；保留手寫 journeys 和清理後證據 |
| 恢復與交付 | 延續既有 attempt 與有效證據；清理只作用於所屬資源，未整合工作保存；local 與 deployed 結果分開 |

評估這次收斂，優先看漏掉的 acceptance、使用過期 context 的次數、能否冷啟動重跑、錯誤完成宣稱，以及為實際問題花的驗證成本。不以新增 skill 數、評分滿分或文件數作成功指標。既有 downstream 失敗記錄能支持改變方向；prototype 與自主驗證的新流程效果仍須上述 pilot 實際觀察。

## 初次研究交付與後續實作

初次研究完成分類與設計提案，逐一對應 L01–L24，並核對現有 8 個 native 入口、context/verification/review 的主要實作與 pstack 三份來源。該研究階段僅更新文件；後續已接受並實作的 preview 行為與獨立 review 見 [preview audit](../audits/2026-09-10-aep-v5-preview.md)。

後續工作是依上面四批實作、補 behavior observations 與必要 deterministic fixtures。新的 review 契約與 artifact coverage 尚未實現；舊 inventory 保留為研究時點的事實快照，移植範圍與工作順序應連同本提案重新判斷。

本次文件驗證：L01–L24 的 ID、順序與名稱和 marketplace 完全一致；四份更新文件共 90 個本機連結可解析，空白格式檢查與 `git diff --check` 通過。重新執行 `cargo run -q -p aep-cli -- skills --json`，仍是原有 8 個入口與原 bundle digest。未變更程式或技能，未重跑 Rust 測試，也未把本提案的行為驗收宣告為通過。
