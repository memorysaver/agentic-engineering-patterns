# MITS reflection → implementation：AEP 5.0 流程觀察

2026-09-10。接續[清除 context 與 reflection 觀察](2026-09-10-mits-cleared-session-observation.md)。使用者自行引導 MITS agent 往下做，要求 AEP 觀察者繼續追蹤；本輪觀察者僅讀取 Herdr 與專案紀錄，沒有提交 prompt、補答案或代為修正。

## 使用者實際引導與範圍

MITS 對話先收到「好我們完成實作」。Agent 區分既有 MITS-108 與 reflection 改善，詢問要完成哪一項，同時檢查兩個 worktree、AEP 狀態與 MITS-108 delivery plan。使用者再指定「這次 reflect 產生的故事是哪一個？自主完成」。

Agent 說明 reflection 當時只有 lesson、沒有 story，並自行收斂成「讓專案狀態對缺少證據的判定與回報範圍一致」，其餘輸出改善保留。此實際引導沒有明確要求 PR、合併或本機 binary 安裝；不能用先前討論的 PR 範例替代這次授權。

## 可觀察的流程交接

1. **既有工作核對。** MITS-108 candidate `c2e390a`、worktree 與歷史證據仍存在。`aep deliver plan` 回報目前 evidence 不符合交付條件；agent 留下 freshness 差異待查，沒有宣称舊證據已足以交付。
2. **反思進入設計。** 自行讀取 design／validate／implement 與 records、BDD、handoff、self-verification 等 references。建立 change `status-evidence-scope` 與 story `MITS-109`，把 source lesson 與驗收範圍接入。
3. **Prototype 回答設計問題。** 使用 installed binary 與 disposable vault：選入 Codex checkpoint 時 pass；三個新 generic checkpoint 排除它後 fail；新增 Codex checkpoint 後 pass。Agent 將此限定為 fixture 的選取效應，沒有聲稱重建 live-vault 的歷史因果。
4. **保留產品契約。** Agent 查到既有意圖針對近期 checkpoints，因此選擇新增 `dogfood.evidence_scope`／`evidence_note`、改善先查證再 capture 的提示，不修改選取演算法、gate 判定、failure ID 或 exit 行為。規格涵蓋 selected／excluded／缺少 checkpoint、未評估、non-MITS 與多格式。
5. **工作容量處理。** 因專案 WIP 1，透過 `aep attempt recover --cancel` 取消 MITS-108 claim，story 回到 pending；原分支、worktree、程式與歷史驗證保留。Agent 將它描述為暫停執行，並記錄恢復需要 current claim／evidence。這不是已整合或已清理。
6. **設計基準。** 主 worktree `MITS/` 切到 `design/status-evidence-scope`，提交 `b26ccabae0ebb02fe1d16482a639ac398641c438`。Spec check、change accept、dispatch plan 與結構檢查均有可見輸出。
7. **隔離實作。** 自行呼叫 `aep dispatch start --story MITS-109 --base b26ccab --owner codex --worktree /home/memorysaver/Work/github/MITS-status-evidence --json`，建立 attempt `attempt-18d400787c18fe82-10c819-0`、分支 `aep/MITS-109-attempt-18d400787c18fe82-10c819-0`，再將 attempt 設為 running。共享 store 仍是 `MITS/`。

## 觀察界線與待核對事項

本輪開始讀取時，agent 已執行約四分鐘；較早步驟從 Herdr 保留的畫面回讀。工具輸出有摺疊，紀錄不是完整 execution trace。

Herdr API 顯示 blocked，畫面同時顯示 Working、持續工具呼叫與「Queued follow-up inputs / 1 question」。這表示不能僅憑 blocked 判定它已停止實作；觀察者沒有對該 UI 代答或操作按鍵。

目前可確認 reflection → design → dispatch → implementation 的紀錄已串起來。MITS-108 freshness 落差、claim 取消與恢復的使用體驗，以及後續驗證／review／收尾是否完整，需分開觀察，不能以建立 worktree 取代完成證據。

## 實作與失敗恢復

Candidate 為 `8d275417989befc25e0762a0ee2da86272cd571a`，修改 status report 與 project-status integration tests；agent 更新 attempt 為 review，再以共享 store 執行 `aep verify run --story MITS-109`。

聚焦測試先通過九項，但第一輪完整 Rust suite 中八項 project-status tests 無法啟動 CLI，回報 `No such file or directory`；native failed evidence `check-18d400a229677a25-10debe-2` 保留。Agent 自行以 `strings` 確認 test executable 仍包含之前的 worktree-local target 路徑：它曾建立 target symlink、之後移除並改用 `CARGO_TARGET_DIR`，重用的編譯產物仍綁舊位置。

Agent 沒有修改產品邏輯，而是觸碰 test source 強制重新編譯，在相同 Git head 重跑 `aep verify run --story MITS-109 --check rust-tests`。新 evidence `check-18d400b7e32b8f2a-111ff4-0` 通過，總計 523 passed、0 failed、1 ignored。觀察者只讀取 evidence 與畫面，沒有提供此原因或修復操作。

先前 Rust-only commit 的 oxfmt hook 問題再次出現；repository check 與 Rust formatter 通過後，agent 使用 `LEFTHOOK_EXCLUDE=oxfmt` 完成該次 commit，沒有修改 hook。這是仍存在的專案操作摩擦，不將成功重試解讀為 hook 已修好。

## 收尾紀錄與證據有效性的交互影響

Agent 加入 change 下的 `verification.md` 後，自行透過 verify plan 察覺：雖然 product commit 沒變，context fingerprint 已變，先前測試／review 不再是目前 context 的證據。它在摘要中明確記錄這個原因，先固定摘要，再重跑 required checks 並重新請 reviewer 對照最終 context；新 evidence ID 留在 native records，不再回填摘要造成第二次失效。

最終重跑的 `verify run` 回報成功。此處是 agent 自行恢復流程的證據，也暴露出保留驗證摘要會觸發額外完整檢查的成本。尚未據此決定哪些文件應排除 fingerprint；不能為降低成本直接放寬證據有效性。

## CLI 呼叫、回傳與可見後續

使用者要求同步觀察 CLI 如何引導步驟。下表分開 skill 內容、命令結果與 agent 後續行為；相鄰的呼叫不是其內部決策原因的完整證明。

| 實際入口／命令 | 可見回傳／作用 | Agent 的可見後續 |
| --- | --- | --- |
| `aep --skill design`，及 `--ref records`／`bdd` | Skill 與 reference Markdown | 建立 change、BDD scenarios、story，保留 prototype 結論與範圍 |
| `aep --skill implement`，及 `--ref handoff`／`git` | 實作與交接指引 Markdown | 核對分支、既有工作、base、WIP 與共享 store |
| `aep --skill validate --ref self-verification` | 可觀察驗證指引 Markdown | 使用隔離 fixture、configured checks 與專案要求的獨立 review |
| `aep spec check --change status-evidence-scope` | PASS，附「context checks 不等於產品行為驗證」說明 | 繼續 accept change，而未把結構通過當作產品驗收 |
| `aep change accept …`／`aep story new --file - --json` | Saved／structured records、changed files、operation identity | 提交設計與 context，作為 dispatch 的 base |
| `aep attempt recover … --cancel` | 舊 attempt／story 更新，worktree 成果保留 | 釋放 WIP 容量，再啟動 MITS-109 |
| `aep dispatch plan --story MITS-109` | Ready，Reasons: none | 使用已提交基準執行 start |
| `aep dispatch start … --json` | 建立 branch／worktree／attempt；回傳 cwd、共享 store、launch request，`worker_started=false` | 同一 agent 將 attempt 標為 running，進入新 worktree 實作，沒有把 prepared 當成 worker 已啟動 |
| `aep attempt record … --status review` | Saved 與受影響 records | 對已提交 product candidate 跑正式驗證 |
| `aep verify run … --json` | 實際執行 configured checks；第一輪 `ok=false`／exit 1 並保存 failed evidence | 查證 stale test executable 的路徑，保留失敗，重編並重跑 Rust checks |
| `aep verify plan --story MITS-109 --json` | candidate head、context fingerprint、checks 與 review 要求 | 發現摘要改變 fingerprint，固定摘要後重跑 required checks |
| `aep review request … --json` | 建立綁定 head／fingerprint 的 review request | 委派獨立 host reviewer，沒有宣稱 CLI 本身執行模型審查 |
| `aep review record --file - --json` | 接受 reviewer 的 structured response（findings 空）並保存 review | 進行 delivery readiness 查詢 |
| `aep deliver plan --story MITS-109` | Eligible: true；四項 check 加一項 review；Missing: none | 保存紀錄後再次確認仍 eligible，回報實作完成但尚未整合 |

這些操作命令没有每次重送 skill 全文。指引由 `--skill` 主動取得；命令回傳執行結果、機械條件、識別資料或特定操作提示。`dispatch start` 的 launch request 另提供「讀 AGENTS／implement、實作指定 story、使用共享 store」的簡短交接；不是自動呼叫下一個 skill 或啟動 agent。

可見命令曾使用大型 JSON 輸出並遭終端摺疊；之後 `verify run` 改為保存 JSON 檔再擷取摘要。這種自行調整輸出處理有助於觀察，但不是 CLI 已全面提供精簡輸出的證明。

## 本輪結果

本輪跨至台北時間 2026-09-11，agent 最終回覆顯示 14m 45s。實作 commit `8d275417989befc25e0762a0ee2da86272cd571a`；設計基準 `b26ccabae0ebb02fe1d16482a639ac398641c438`；共享帳本／證據 commit `21347f7562d29a14ce25b0dce124631b88ebab12`。主 worktree 與 implementation worktree 末次檢查皆乾淨。

最終 fingerprint `e73368ee8d68f0a14c18761fe4e5943e6c5792912dc47430acae57d539ecf2c9` 下，local-preflight、repo-check、rust-tests、secret-scan 均 pass，Rust 為 523 passed／1 ignored；獨立 review `review-18d400ebbd888542-11908b-0` 已登錄。觀察者在證據提交後另行讀取 delivery plan，仍是 Eligible: true、Missing: none。

Agent 保存 progress record `20260910T160051Z573135044` 並 sync；這輪不是唯讀實驗。未建立 PR、合併、發布 spec 或替換 PATH binary。這反映 agent 將「自主完成實作」解讀為 candidate completion；不能據此認定使用者期待的 AEP 完整收尾已滿足，也不把交付資格當成已整合。MITS-108 原有分支／worktree 仍保留，其 freshness／恢復問題未在此解決。

結論限於此案例：在使用者釐清目標後，agent 自行完成 reflection → design／prototype → dispatch／隔離實作 → 驗證／失敗恢復 → 獨立 review → readiness 檢查。觀察者沒有代補操作指令。真正的 PR／integration／cleanup 銜接尚未在本輪執行，因此不宣稱整個交付生命週期已驗證。

## 目錄分類核對

使用者另問資料是否寫回 v5 規劃的分類。觀察者比對 `1d775db..21347f7` 的 shared-store diff、implementation 的 `b26ccab..8d27541` diff 與 `.aep/config.toml`：

- 原生 lesson 在 `lesson-learned/observations/`；長篇 reflection 與篩選後 JSON 在同一 lessons store 下的 `reflections/`，由 lesson 的 data 與內文指向。
- Story、attempt、events、check evidence、review 分別在 `project-ledger/stories/`、`attempts/`、`events/`、`evidence/`、`reviews/`。
- Change contract、BDD delta、prototype 與驗證摘要在 `project-ledger/changes/status-evidence-scope/`。
- Product code／tests 只在 implementation worktree 的兩個既有 Rust 檔案；shared store 保存上述紀錄。兩部分各有已提交的 commit，尚未進 main。
- 沒有更新 legacy `product-context.yaml`、`lessons-learned/` 或 `.dev-workflow/`。未採納新規則，未發布已整合規格，因此沒有要求本輪修改 `project-rules/` 或 `project-roadmap/specs/`。

`aep check` 與 `aep migrate verify` 重新讀取後皆 PASS（330 records）。這支持 native records 的結構／路由與 legacy 來源保留，不代表所有手寫文件分類都已被 CLI 強制驗證。

需要保留的規約問題：agent 將 design rationale 寫成 change 內的 `design.md`，沒有新增 `docs/design/` 草稿或 `project-roadmap/decisions/` ADR。Native design skill 指示 drafts／accepted tradeoffs 的落點，但 CLI workflow guide 又允許 change 含 design prose；不能僅憑本輪沒有獨立 ADR 就定為違規。應釐清何時 change 內文已足夠、何時需獨立草稿／decision，並保持文件與入口說明一致。本輪没有移動檔案或事後補造 ADR。

## 使用者指出的交付收尾缺口

使用者觀察「做完好像沒有自主 wrap，像 4.x」。對照 source 後，4.x build 的輸出包含 PR／merge，wrap 在 merge 後承接 archive、story 狀態、證據與 lessons 收斂，以及 worker／worktree 清理；interactive／autopilot 的合併權限不同。5.0 將這些交付責任集中到 deliver／closure，沒有宣告取消收尾。

本輪 agent 讀過 deliver，也跑出 Eligible: true，卻選擇把完成界線停在可交付 candidate。它已保存證據與記憶，但没有 PR／integration receipt、spec publication、change closure 或 worktree cleanup，MITS-109 仍 in_progress。直接刪除未整合 worktree 當然不是正確補救；真正待驗證的是 implement／validate 如何接入使用者預期的 delivery endpoint，再於整合後完成 closure。

觀察者先前將「未交付」描述成符合授權界線，過早替 agent 的終點選擇背書。使用者的實驗目標是檢驗流程整併後仍能自主維持 AEP 全生命週期；目前只能證明到 delivery readiness，並已出現使用者預期與 agent 自訂 completion boundary 不一致的證據。此處記為流程引導／交付終點的缺口，沒有代替 MITS agent 建立 PR、merge 或清理 worktree。
