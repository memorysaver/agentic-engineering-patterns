# MITS v5 狀態盤點：Herdr 實際觀察

2026-09-10。使用者要求透過 Herdr 操作 MITS workspace，從「我們現在做到哪裡了」開始了解專案狀態。後續明確指示：發現問題先記錄，繼續觀察，稍後再修正。本紀錄不構成修正或新功能開發授權。

## 觀察方式

- Workspace `MITS`，pane `w4:p2`，已存在且閒置的 Codex agent；畫面顯示 gpt-6-astra high。
- 專案為 `main`，HEAD `67d9db9dc210a65036be905b4e8bd74949bca87e`，起始工作目錄乾淨。
- 透過 `herdr agent prompt` 要求盤點完成事項、未完成工作、阻擋、待使用者決定事項及建議下一步，明確限定本輪不開始實作新功能。沒有先提供狀態答案或指定它應發現哪些問題。
- 以 `agent get` 與 `agent read --source recent-unwrapped` 觀察行為。第一次 50 秒 wait timeout 時 agent 仍為 working；沒有重送 prompt，也沒有將 timeout 當成 agent 失敗。
- 此處記錄可見的工具呼叫、輸出與 agent 判讀。CLI 輸出有摺疊、alternate-screen 捲動限制，故不聲稱是完整工具追蹤。

## 問題與自行恢復情況（均未修正）

| 編號 | 實際觀察 | 影響／目前判定 |
| --- | --- | --- |
| OBS-01 | 先呼叫 `aep --skill roadmap references/status.md`；再試 `aep --skill roadmap --ref references/status.md`。第二次錯誤列出可用名稱 `product-context, status`，之後自行改成 `aep --skill roadmap --ref status` 成功。 | Skill 的相對 Markdown 路徑和 CLI reference 名稱不同，造成兩次可恢復的操作錯誤。須後續評估呈現／尋找方式，未選定修法。 |
| OBS-02 | 直接以 Python 解析 story YAML 時缺少 yaml module；改用 Ruby 時遇到受限日期／時間類型載入錯誤。後來使用 CLI JSON，也以明確允許 Time/Date 的 Ruby 讀取。 | 狀態盤點引入額外解析器與環境依賴。這是本次 agent 行為與使用摩擦，不代表 CLI 本身無法讀取資料。 |
| OBS-03 | 一次將完整 `aep status --json` 重印，終端顯示超過 3,000 行摺疊輸出；稍後才依 `data.stories` 過濾。`mits orient --format json` 也有超過 950 行摺疊輸出。 | 可見大量資料輸出與後續過濾；尚未量測實際 token、延遲或錯誤率，不能只憑畫面判定 context 遺失。 |
| OBS-04 | Agent 回報 `mits project status --deep` 的 `adoption_missing_agents_trigger`，並讀取 `crates/mits/src/project/adoption.rs`、精簡後的 AGENTS.md 與 `project-rules/mits-memory.md` 比對。 | 具體接入契約不一致：健康檢查仍要求記憶指令直接出現在 AGENTS.md，而 v5 入口將專案指引移至 indexed rules。程式與文件均尚未修改。 |
| OBS-05 | Agent 回報 recall 包含未記錄結案的歷史 blocker，並交叉比對後續 Git 合併與 Layer 18 結案證據。 | 記憶的歷史阻擋與當前可執行工作需要辨識；此觀察不等於已確認所有舊 blocker 都失效。沒有修改或批次結案記憶。 |
| OBS-06 | 沒有程式變更時，仍重複執行 help_smoke 與 fmt 檢查。 | 可見重複驗證，後續可評估狀態盤點的驗證範圍；目前沒有建立固定次數或新流程要求。 |

## 有效行為與界線

Agent 自行使用 `aep --skill`、roadmap、verify-mits，讀取專案規則與 Git／worktree／PR 狀態；也執行 MITS orient、recall、deep project status。Reference 使用錯誤後，依 CLI 回饋自行修正，觀察者沒有提供修正答案。

它區分 Git 的歷史合併證據和 v5 的 imported／尚未重新驗證狀態，也保留 Layer 19 尚待使用者選擇的限制。畫面可見 preflight、repository check、CLI help smoke、fmt、歷史 portable evidence verifier、aep check 與 migration verify 的結果；這些不是所有歷史產品能力的驗證。

本輪並非完全唯讀：agent 執行了 `mits sync`，並在盤點末尾 `mits remember` 後再次 sync。這些是記憶庫操作，應與專案程式／帳本是否有修改分開記錄。Agent 顯示的末次 Git status 為乾淨。

## 本輪收尾

Herdr 最後回報 idle，畫面顯示本輪耗時 3m 56s。已讀到完整最終回答，未啟動修正或新功能任務。

Agent 的盤點摘要：近期完成 MITS-105 embeddings reuse、MITS-106 cache budget 修復、MITS-107 current-state recall 精簡與 PR #100 v5 migration。Layer 19 仍暫停。AEP 狀態為 96 imported、7 deferred、4 pending、0 ready；它沒有將 imported 解讀成產品從未實作，也沒有直接修改完成狀態。

另有待查觀察：最終回答回報 13 筆缺少有效結案關係的歷史事項、daemon 暖快取 stale，以及 recall 的 claims／duplicate-collapse 上限警告。這些是 agent 從本輪資料整理的發現，觀察者未另外重跑或確認每項原因，不把它們自動歸因於 v5 migration。

它提出先處理 adoption 相容性，再整理過期記憶／所需依賴證據，最後評估 MITS-072 的建議。此順序只是 agent 的建議，沒有被執行，也不代表使用者已選定下一個產品切片。

畫面回報通過：AEP 結構／遷移、local preflight、repository check、fmt、10 個 CLI smoke tests、31 項 Layer 18 retained-evidence checks。完整 crate suite 沒有重跑；agent 明確將 520 passed／1 ignored 標為 MITS-107 既有證據。本輪末尾記憶 checkpoint 為 `20260910T140632Z509020399`，已 sync。

## 後續處置

本輪觀察已完成，保留上述問題供後續討論。上述項目均保持待評估／待修正；本紀錄不改 CLI、skill、AGENTS.md、MITS 程式碼或 native 帳本。

後續使用者另行授權修正已知錯誤；處置與驗證另記於[修正紀錄](../audits/2026-09-10-status-observation-fixes.md)，上述觀察保留本輪當時狀態。
