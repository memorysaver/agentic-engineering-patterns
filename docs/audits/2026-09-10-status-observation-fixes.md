# MITS 狀態觀察後的已確認錯誤修正

2026-09-10。使用者在[原始觀察](../lessons/2026-09-10-mits-v5-status-observation.md)完成後授權「先修正已知的錯誤」。本次處理 OBS-01 的 CLI reference 契約，以及 OBS-04 的 MITS adoption 相容性；原始觀察保留當時狀態。

## AEP reference 尋找與執行

實作 commit：`0047449f452cbe3809fbd721ef363f370c0d9177`。

`aep --skill roadmap --ref status` 與 `aep --skill roadmap --ref references/status.md` 現在取得相同原始 Markdown。CLI 只比對已登錄 reference 的名稱與對應路徑，不開放任意檔案讀取。不存在的 reference 會列出可直接執行的完整命令。

內建 skill 的 reference 連結旁提供 CLI 命令，JSON reference metadata 提供對應 `path`。仍只保留 `aep --skill` 一個入口；位置參數 `aep --skill roadmap references/status.md` 並未新增為另一種語法。狀態指引補上範圍查詢與 JSON 欄位範例，降低直接解析 YAML 或重印完整狀態的需要；尚未重新觀察 agent 是否因此改變行為。

驗證：70 個 Rust tests、fmt、Clippy、四項技能套件檢查、九個 native skill frontmatter 驗證均通過。測試涵蓋所有內建 reference 的兩種用法、project-owned reference，以及未知／越界路徑拒絕。獨立複製的 release binary 在臨時 legacy 專案完成初始化、migration、重複 apply、原始資料保留與 native 寫入試跑；三個 downstream 的 reference、結構與 migration 驗證亦通過。

本機已安裝 preview `5.0.0-preview.1`；未發布新的公開版本或 tag。

- Binary：`~/.local/share/aep/builds/b0f05d65ce213d99/bin/aep`，`~/.local/bin/aep` 指向此產物。
- SHA-256：`b0f05d65ce213d998a37ec83b0738411e6af9795fceb88097e33486145c218b9`。
- 本機試跑證據：`~/.local/share/aep/trials/2026-09-10-reference-links/summary.json`。

## MITS adoption 與精簡 AGENTS 的相容性

透過原 MITS Herdr agent 執行，story `MITS-108`；implementation worktree 為 `MITS-adoption-fix`。修正 commit：`185ac2eea70c9e1f8516f8df403ecfedc472241e`，review 後修補 commit：`c2e390ac0ab4afba5325d2d299c672e0e46c17cc`。

產品修正在 `aep/MITS-108-attempt-18d3fb19e1b60eaa-eb700-0`；共享帳本與驗證紀錄在 `fix/adoption-guidance-context` 的 `1d775db`。MITS 的 `project-ledger/changes/adoption-indexed-guidance/verification.md` 索引保留的測試、獨立 review、實際安裝前後與回滾證據。Native 命令使用共享根目錄 `MITS`，worktree 內的 story 是基底快照。

Adoption 檢查保留既有完整 AGENTS 內文的支援，也辨識明確的 `AGENTS.md → project-rules/README.md → mits-memory.md` 路徑。三份文件均保持原樣。檢查會讀取可達的規則內容；只有檔案存在、範例文字或指向專案外的 symlink 並不足以通過。已有索引但路徑缺漏時，install-memory 保留文件並要求修復索引，不將完整政策重新附加到 AGENTS。

第一輪獨立 review 找出範例連結誤判與合法 Markdown 標題／角括號連結漏判，均補上正反例測試。第二輪 reviewer 以 11 個 CLI probes 確認修復。最終 candidate 完整 Rust suite 為 526 passed、0 failed、1 ignored；repository check、fmt、local preflight 與 secret scan 通過。既有 oxfmt pre-commit hook 會誤處理 Rust-only staged files，本次先通過 repository check 與 Rust fmt，僅排除該次 oxfmt hook；沒有修改 hook 設定，此接入問題保留待處理。

Candidate CLI 在隔離 vault／repository 載入真實三份指引後，audit、install 與 deep status 通過且文件 digest 不變；移除 memory rule 後，deep status 精確恢復 `adoption_missing_agents_trigger`，install 拒絕並保留 AGENTS。第一次試跑 script 將 install 拒絕 exit code 寫為 6，實際契約為 70；已保存失敗輸出並修正 script 後重跑，沒有為此修改產品行為。

Review 結案時另觀察到 agent 將敘述文字放進 `finding.evidence`，CLI 拒絕；此欄位要求目前 fingerprint 下通過的 ledger Evidence ID。觀察者讀取 AEP 實作後提供此契約與正確 source repository 路徑，沒有繞過 gate。這段不屬於未介入的自主操作觀察。

### 本機安裝與實際專案結果

保留既有 `experimental-local-embeddings` 功能，以 Rust 1.95.0 建置 release，更新 `/home/memorysaver/.cargo/bin/mits`。未 merge 或公開發布。

- 新 binary SHA-256：`80cbf44ed2bd4adc60f511604eba1f7ec85ee0539c038ec44ab3329fbc64b87d`。
- 舊 binary SHA-256：`f253df1b5b137a664dc41558e38fca952f9ee32cf46a67fcf25c4e5ffeaa9f7c`。
- 備份、manifest、before/after audit 與 deep status：`~/.local/state/mits/binary-backups/20260910-adoption-wrge3phk/`。
- 回滾工具：同目錄 `rollback.py`；dry validation 通過，沒有實際回滾本次修正。

實際 MITS repository：`agents_trigger_present` 由 false 轉為 true，deep status 的 `hard_failures` 從僅包含 `adoption_missing_agents_trigger` 變為空陣列，`quality.overall_pass` 由 false 轉為 true。原有 claims sidecar byte limit 與 `duplicate-collapse=limit_absent` 警告仍存在。本輪證據並不表示歷史 blocker 或 recall 上限問題已修復。

## 尚未確定原因的觀察

舊記憶 blocker、daemon 暖快取 stale、recall 上限警告尚未完成原因確認；本次沒有自動清理、結案或把它們歸因於 migration。解析器缺漏、大量輸出與重複驗證則保留為行為觀察，沒有加入固定 agent 執行順序或驗證次數。
