# HEIC → JPG Converter

**外部依存ゼロ**のデスクトップアプリ。ImageMagick等のインストール不要。
Mac (.app) と Windows (.exe/.msi) の両方に対応。

---

## 仕組み

| コンポーネント | 役割 |
|---|---|
| **Tauri 2** | クロスプラットフォームGUIフレームワーク |
| **libheif-rs** | HEICデコード（Rustバインディング） |
| **image crate** | JPEGエンコード（Pure Rust） |

ビルドしたバイナリ1つで動作します。ユーザーは何もインストール不要です。

---

## ビルド手順

### 必要な環境（開発者のみ）

```bash
# 1. Rust をインストール（まだの場合）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Node.js v18+ をインストール
#    https://nodejs.org/

# 3. macOS の場合 — libheif のビルドに必要
brew install cmake pkg-config libheif

# 4. Windows の場合 — vcpkg で libheif をインストール
#    https://github.com/microsoft/vcpkg
vcpkg install libheif:x64-windows
```

### ビルド

```bash
cd heic-converter
npm install
npm run build
```

完了すると以下に出力されます：
- **macOS**: `src-tauri/target/release/bundle/macos/HEIC Converter.app`
- **Windows**: `src-tauri\target\release\bundle\msi\HEIC Converter_0.1.0_x64.msi`

---

## 使い方

1. HEICファイルをウィンドウにドラッグ＆ドロップ
   （または「ファイルを選択」ボタン）
2. JPG品質をスライダーで調整（デフォルト: 90%）
3. 「変換開始」をクリック
4. 元ファイルと同じフォルダの **`jpg_output/`** に保存されます

---

## トラブルシューティング

### macOS: `libheif not found`
```bash
brew install libheif
```

### Windows: `LIBHEIF_DIR not found`
vcpkg でインストール後、環境変数を設定：
```powershell
$env:LIBHEIF_DIR = "C:\vcpkg\installed\x64-windows"
```

### Rustコンパイルエラー
```bash
rustup update
cargo clean
npm run build
```
