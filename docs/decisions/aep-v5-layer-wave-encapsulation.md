# AEP v5：以 layer／wave 封裝同一概念的設計與工作

狀態：指引部分已接受並實作（2026-09-11，使用者同意後修改 native skill、重建並安裝 CLI）；CLI 候選變更（check 診斷、status 分組）仍為待決。使用者指出目前設計「只有開 story」，要求以 layer 與 wave 封裝同一概念的設計以保證關聯性，並將此內建到 skill 流程。
日期：2026-09-11
證據：[MITS 故事規劃觀察](../lessons/2026-09-11-mits-story-planning.md)、[MITS layer／wave 引導觀察](../lessons/2026-09-11-mits-layer-wave-steering.md)
相關：[產品目的驅動研究](aep-v5-purpose-driven-research.md)、[研究與草案 context](aep-v5-research-draft-context.md)、[skill 分類決策](aep-v5-context-and-self-verification.md)

## 問題

MITS agent 依一份 design 草案建立了九個 story、九個 change 與一個 roadmap 記錄。它用 roadmap 的 `data.recommended_execution_order`、每個 story 的 `data.recommended_order` 與 `data.design_artifact`，加上 `depends_on` 鏈，表達「這九件事屬於同一個概念」。所有 story 的 `layer` 與 `wave` 都是 null。使用者主動詢問後，agent 立即提出一個 layer 加三個 wave 的分組與 layer 完成條件，但仍未寫入記錄。

概念沒有原生容器時，關聯性只存在於 prose 與任意 `data` 欄位。Runtime 讀取容器的機制看不到這個群組：readiness 不會繼承群組的依賴與 gate，gate 無法以群組為範圍，`context` 與 `status` 無法從概念層級展開，後續 agent 只能靠 roadmap 的 refs 反推成員。

## Runtime 已有的容器語意

以下為 `aep-core` 與 `aep-cli` 目前行為，來源為 `crates/aep-core/src/lib.rs` 與 `crates/aep-cli/src/commands.rs`：

| 能力 | 目前行為 |
| --- | --- |
| 記錄形狀 | story 有 `layer`、`wave`、`release` 欄位；容器以 `refs` 列成員。`scope_stories` 從兩個方向認定成員 |
| 連結與 context | `links()` 包含 `layer`、`wave`；`aep context <id>` 沿這些連結展開，容器與成員互相可達。`validate` 檢查 `missing_reference` 與 `reference_kind` |
| readiness | story 繼承所在 layer／wave 的 `depends_on` 與 `required_gates`。容器作為依賴時展開為成員 story，每個成員需 integrated 且有 delivery |
| gate | `gate.refs` 指向容器即涵蓋全部成員，用於 `gate evaluate` 與 dispatch |
| 遷移 | legacy layer 變成 `layer-N`，wave 變成 `wave-N-M`，layer 邊界變成 `gate-layer-N-N+1`；legacy 的 `theme`、`outcome` 只保留在 `data.legacy_metadata` |
| 狀態呈現 | `aep status` 依 story 列出 readiness，不按容器分組，也不顯示容器 outcome |

容器記錄本身沒有 outcome、設計來源或完成條件欄位。MITS 遷移後的 20 個 layer 與 75 個 wave 的 `description` 均為空。

## v4 的概念與 v5 分類決策

v4 的 `map` skill 把 layer 定義為「名稱 + outcome + stories」，wave 是同一 layer 內可平行派工的批次，每個 layer 有 gate 與 outcome contract，由 reflect 在 layer 完成後評估。MITS 的 `project-roadmap/product/maps/project-state-memory/map.yaml` 保留了這種結構，例如 Layer 3「Cross-Project Lessons and Preference Recall」有可觀察的 outcome 與六個成員 story。

v5 的分類決策移除了數字 layer 的隱含流程與 `.5` gate，並以明確依賴取代 layer／slice 排序語意。這個決策沒有同時說明「概念容器」的責任要落到哪裡。結果是 native 指引只剩 roadmap skill 的一句「Layers organize a theme or owner; waves/releases organize coordination and delivery」，records 參考只有單一 change 加單一 story 的範例，design skill 寫「author the change contract and link its story」也是單數。沒有任何指引說明何時該建容器、容器要帶什麼、容器如何連回 design 與 decision。

## 提案行為

**觸發。**當一份 design 或 decision 拆成多個 story 或多個 change 時，先建立一個 layer 作為概念容器，再建立 change 與 story 並設定 `story.layer`。需要先後批次或平行協調時再建 wave 並設定 `story.wave`。單一 story 不需要容器。這個判斷由 working agent 做，指引描述觸發條件與內容契約。

**容器內容契約。**layer 的 `title` 是概念名稱；`description` 是可觀察的 outcome，也就是這組工作全部整合後使用者能看到的差異；`refs` 列成員 story 與來源 decision／roadmap；`data` 記錄 design 草案路徑與研究來源；有整合驗證需求時設定 `required_gates`；對其他概念的前置關係放 `depends_on`。wave 的 `refs` 列成員，`depends_on` 指前一個 wave，`description` 說明這批的意圖與完成後的狀態。

**建立順序。**decision → design 草案 → layer → change（含 spec）→ story（設定 `layer`／`wave`）→ wave。roadmap 記錄的 `refs` 指向 layer，不再於 `data` 重複列 story 順序。順序與批次由 `depends_on` 與 wave 表達，不用 `data.recommended_order`。

**完成判定與回報。**layer 完成等於成員 story 全部 integrated、必要 gate 當前，且 outcome 有觀察證據。status 摘要與 handoff 以容器為單位說明現在位置、成員狀態、outcome 與剩餘工作。reflect 在 layer 完成後對照 outcome，不重建 v4 的固定 outcome contract 流程。

**指引落點。**維持八個入口，不新增 skill。已修改 `roadmap/SKILL.md` 的容器定義句、`roadmap/references/product-context.md` 的拆工段落、`design/SKILL.md` 的多 story 情形、`design/references/records.md` 的 layer／wave 輸入範例與建立順序、`roadmap/references/status.md` 的按容器回報，以及 `implement/references/handoff.md` 的 layer outcome 交接。每處一到三句，未新增 NEVER／MUST／ALWAYS 類硬性指令。實作與安裝證據見 [MITS layer／wave 採用觀察](../lessons/2026-09-11-mits-layer-wave-adoption.md)。

## CLI 候選變更

這些是待決選項，需與指引一起評估：

- `aep check` 增加診斷：layer／wave 沒有成員；story 的 `change` 所屬 design 或 roadmap 下已有多個 story 但無容器。後者可能誤報，建議先做前者，後者以 policy 開關或 warning 等級處理。
- `aep status` 按 layer 分組顯示成員 readiness 與 outcome，並保留現有依 story 的輸出。
- `aep context <layer>` 已能展開成員；若採納研究草案 context 提案，容器的 `data` 設計路徑可透過同一機制載入內容與 digest。
- 不新增 record kind。容器的 outcome 與設計來源以 kind-specific `data` 欄位表達，`validate` 只檢查存在與型別。

## 開放決策

- 指引與 policy 的分工：是否允許專案在 `.aep/config.toml` 要求多 story 必須有容器，或只保留指引。
- wave 的必要性：v5 已有 `depends_on` 與 `max_parallel`；建議 wave 只在需要批次協調時使用，預設只用 layer。
- roadmap 記錄與 layer 的分工：roadmap 表達方向與優先序，layer 表達可完成的概念；兩者的 refs 方向要在指引中固定。
- 遷移產生的 `layer-N` 空描述是否回填 legacy 的 name／outcome。回填只影響描述，不改變已 imported 的完成主張。
- MITS-110 至 118 可作為第一個採用案例：在使用者授權下建立 layer 與 wave、更新 story 欄位並移除 `data` 中的順序欄位。這是下游工作，不在本提案內執行。

## 驗證方式

- 隔離情境：一份 design 拆成三個以上 story。觀察 agent 是否未經提示建立 layer、設定成員、寫出可觀察 outcome、把 roadmap refs 指向 layer，以及 handoff 是否以容器說明位置。
- Runtime：`aep context <layer>` 列出成員、change 與來源 decision；成員 story 的 readiness 繼承容器依賴與 gate；`aep check` 對無成員容器產生診斷；`aep status` 分組正確。
- Fixture：Rust 測試涵蓋新診斷與 status 輸出；skill 文字 fixture 驗證安裝輸出。
- 下游：以 MITS 的實際採用觀察指引是否足以觸發，並記錄 agent 有無再次依賴 `data` 順序欄位。

結構通過不證明分組正確；分組是否對應同一概念，仍需對照 design 與 decision 的語意。
