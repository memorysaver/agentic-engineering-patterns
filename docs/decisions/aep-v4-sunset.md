# v4 與 v5 分線：v4 逐步 sunset

日期：2026-09-18。狀態：**已接受**（使用者決定：v5 是新的 CLI-based 版本，v4 逐步 sunset）。
相關：[preview 採用與雙版本並存](aep-v5-preview-adoption.md)、[Rust CLI 架構](aep-v5-rust-cli-architecture.md)、[legacy v4.1 指南](../workflow/aep-v4.1-guide.md)

## 決定

1. `main` 是 v5 的開發與發布線。第一個發布是 `5.0.0-preview.1`，之後的 v5 版本都從 `main` 打 `v5.*` tag，由 `.github/workflows/native.yml` 產出 Linux x86_64 與 macOS Apple Silicon 的壓縮檔與 checksum。
2. v4 從 `v4.1.0` tag 切出 `v4` 分支保存。既有安裝可以繼續用該 tag 與 `.claude-plugin/marketplace.json` 釘住的 4.1.0；v4 不再加新功能，只在使用者明確要求時修安全或阻斷性問題，改動只進 `v4` 分支並打 `v4.x` tag。
3. v4 與 v5 不互相自動切換。2026-09-10 的並存決策仍適用：專案明確選用的版本優先，缺能力要明示，不偷偷 fallback。
4. GitHub release 的 `latest` 標記：v5 的 prerelease 以 `--latest=false` 發布，`latest` 暫時仍是 v4.1.0。發布第一個 v5 stable 時改標 v5 為 latest，v4.1.0 保留為歷史 release。
5. Sunset 的完成條件：downstream 專案（目前為 MITS、looplia、Rewarc-AutoResearch）都在 v5 上完成至少一次交付，且 v5 stable 已發布。屆時把 v4 的安裝說明從 README 移到 `docs/workflow/aep-v4.1-guide.md`，並在 marketplace 標註停止維護；不刪除 tag、分支或文件。

## 不做的事

- 不把 v4 的 skill 內容遷入 v5 binary；v5 的指引已在 `skills/native/` 獨立維護。
- 不刪除或改寫 v4 的歷史 release、CHANGELOG 條目與文件。
- 不在 v5 preview 階段更動 `latest` 標記。

## 執行紀錄

- 2026-09-18：PR #35 以 merge commit `31f4d42` 合併到 `main`；`v4` 分支建立於 `acf03fc`（v4.1.0）；tag `v5.0.0-preview.1` 指向 `198c2fe`。GitHub Actions run 35301781613 三個 job 全部成功，release 以 prerelease 發布，`latest` 仍是 v4.1.0。資產：`aep-x86_64-unknown-linux-gnu.tar.gz`（1.55 MB，sha256 `f104dd03…`）與 `aep-aarch64-apple-darwin.tar.gz`（1.39 MB），各附 `.sha256`。用 `main` 上的 `scripts/install.sh` 從真實 release 安裝到暫存目錄驗證：自動選到 v5.0.0-preview.1、checksum 相符、binary `d083229a…` 可執行、`--note` 存在、對 MITS 唯讀 `aep check` 548 筆通過。macOS Apple Silicon 的資產只有 CI 建置證據，尚未在實機安裝驗證。
