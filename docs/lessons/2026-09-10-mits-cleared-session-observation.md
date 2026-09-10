# MITS 清除 context 後的第二次狀態盤點

2026-09-10。使用者在 MITS 清除 conversation context 後重新詢問「目前專案進度」，要求追蹤。此次沿用已修正並安裝的 AEP／MITS CLI，背景見[修正紀錄](../audits/2026-09-10-status-observation-fixes.md)。

## 觀察範圍

透過 Herdr 讀取原 workspace 的 `w4:p2`；畫面為新對話，模型 gpt-6-astra high。觀察者沒有提交 prompt、提供答案或介入 agent 的本輪執行。Agent 從 working 轉為 idle，完整最終回覆可讀，畫面顯示 `Worked for 1m 11s`。

專案起訖為乾淨的 `fix/adoption-guidance-context`，HEAD `1d775db451bbdbb64e7b2311d84a2edac34907e3`。終端可見的 204,911 token 摘要屬於前一個 session，不當作本輪用量。原始畫面存在工具輸出摺疊，不聲稱取得完整工具 payload。

## 可見行為與結果

| 面向 | 本輪實際觀察 |
| --- | --- |
| 入口與 context | 自行執行 `aep --skill`、讀 README、roadmap 與 mits-memory skill，讀 migration 規則、CLI config、memory 規則，核對 Git branch 與 log。 |
| Reference | 一次使用 `aep --skill roadmap --ref status` 成功，沒有重現先前兩次 reference 語法錯誤。這輪沒有測試 linked-path 用法。 |
| 狀態範圍 | 先用人類可讀的 `aep status`，再用 `aep context MITS-108`、`aep timeline`，並讀取該 change 的 verification 與 story。沒有可見的完整 AEP status JSON 重印或額外 YAML parser 錯誤。 |
| 證據辨識 | 正確指出 MITS-108 已實作、獨立 review 與本機試用，但尚未 PR／merge，story 仍 in_progress。526 passed／1 ignored 明確標為既有驗證，沒有聲稱本輪重跑。 |
| 自我核對 | 執行 `aep check`、memory sync、orient、recall、deep status，並用 `mits show` 核對前兩筆 checkpoint。發現本次 deep status 與前次成功紀錄不同，以本次結果為準。 |
| 範圍控制 | 保留 Layer 19 暫停與待選題狀態；沒有自行開始產品實作，也沒有為盤點重跑完整測試。 |
| 剩餘輸出摩擦 | `mits orient --format json` 顯示超過 968 行摺疊輸出，deep status JSON 超過 412 行；recall Markdown 超過 176 行。這只是輸出量觀察，未量測 token 或證明造成 context 遺失。 |

本輪 agent 有執行 `mits sync`，因此不是完全唯讀；沒有可見的新 checkpoint 寫入。觀察者在它結束後僅讀取狀態、程式和既有記憶 metadata，沒有修正 MITS 或記憶內容。

## 新發現：dogfood 證據被 checkpoint 視窗排除

Agent 最終回報 `mits_codex_dogfood_evidence_missing`；觀察者另外執行相同專案的 `mits project status --deep --format json`，於 `2026-09-10T14:50:06Z` 確認：

- `adoption` 三項皆 true，`blocking_defects=[]`；先前 adoption 修正仍有效。
- `hard_failures=["mits_codex_dogfood_evidence_missing"]`，`quality.overall_pass=false`。
- `dogfood.checkpoint_count=3`，`codex_checkpoint_record_ids=[]`，`source_profiles=[]`；packet usefulness 仍 pass。
- claims sidecar byte limit 與 `duplicate-collapse=limit_absent` 警告仍在。

唯讀核對的最新 checkpoint 順序：

| 順位 | Record ID | Codex source profile |
| --- | --- | --- |
| 1 | `20260910T142553Z769024524` | 無 |
| 2 | `20260910T140632Z509020399` | 無 |
| 3 | `20260910T105014Z077349843` | 無 |
| 4 | `20260909T083142Z934555326` | `codex` |

MITS `crates/mits/src/packet/mod.rs` 的 `memory_checkpoint_context` 只保留最近三筆（`.take(3)`）；`crates/mits/src/status/project.rs` 的 `codex_checkpoint_record_ids` 只檢查 packet 內的 checkpoint，`evaluate_packet` 在沒有匹配時產生 hard failure。較舊、帶 Codex 標記的證據仍存在於 vault，但落在這個視窗外。

前一輪成功的 deep status 在收尾 checkpoint `20260910T142553Z769024524` 寫入之前；該新增紀錄沒有 source profile，將原本第三筆的 Codex 證據擠到第四筆。程式條件與現有資料足以解釋 pass → fail 的落差；沒有證據顯示 clear 刪除了記憶或 adoption 修正回歸。

此處記錄兩個相關缺口：健康檢查將有限的呈現視窗當成證據全集，以及前幾次 Codex checkpoint 沒有保留來源標記。尚未選定修法，沒有新增假證據、補寫既有 record 或放寬 gate。

## 判讀界線

本輪比第一次盤點的 3m 56s 短，但修正版 CLI、已新增的 verification 文件與 memory checkpoint、對話 context 均已改變；不能將時間差單獨歸因於 CLI 或 skill 改善。可確認的是 reference 讀取順利、整合狀態辨識正確，以及 agent 主動找出新的即時驗證落差。

## 使用者校正觀察者的角色

使用者隨後明確指出：應引導 MITS agent reflect，不應由外部觀察者直接代為診斷或指定修正。上述根因核對是觀察者額外做的工作，不是 MITS agent 的自主反思成果；這也是本次觀察方法需要修正的地方。

後續透過 Herdr 要求 MITS agent 使用 `aep --skill reflect`，回顧自己的 context 取得、證據判讀、驗證落差與操作摩擦，自行選擇有限查證並保存 lessons。沒有提供上述 checkpoint 視窗診斷；交付限定 reflection 與紀錄，不實作產品修正、放寬 gate 或補造通過證據。應評估它如何依現有 context 與 self-verification 指引形成判斷，再據此改善 AEP。

### Agent 自行 reflect 的結果

Herdr 最終回報 done，畫面顯示 5m 21s。Agent 自行發現前後 binary hash 相同、記憶筆數由 273 增為 274，並讀取程式確認 dogfood 判定使用 packet 選入的 checkpoints。它保留「選取內容或 metadata 影響結果」為假設，指出尚缺歷史選取清單與受控比較，沒有把它提升成已確認 regression。

它反思自己對「以本次結果為準」與「缺少證據」的表述範圍，並認為即時檢查前應讀 verify-mits。三項 lessons 為：比較時標明版本、時間及輸入範圍；摘要缺席不等於完整資料不存在；先檢視自己的查詢與推論，再決定修正建議。沒有將三項 lessons 自動採納成規則。

本輪另觀察到 AEP 摩擦：`aep lesson find evidence` 顯示超過 1,262 行摺疊輸出；agent 為 lesson 輸入格式搜尋 help／既有紀錄，初次 dry-run 因缺少 `kind` 失敗，自行依錯誤訊息補上後成功，觀察者未提供格式答案。

MITS 新增 `lesson-learned/reflections/2026-09-10-status-context-evidence.md` 與原生 lesson `lesson-learned/observations/status-context-evidence-reflection-20260910.md`，連同觀察 JSON／事件共四個檔案。Agent 回報 aep check（303 records）、bun run check、讀回與 JSON／連結／空白檢查通過；未跑 Rust suite、未改產品／gate、未新增 live-vault checkpoint。這些反思檔案留在 MITS 工作目錄，尚未提交；觀察者沒有接手修改或提交它們。
