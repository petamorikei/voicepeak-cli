# voicepeak-cli

**Japanese | [English](README.md)**

プリセット管理と自動音声再生機能を備えた、VOICEPEAK音声合成ソフトウェア用のコマンドラインインターフェースラッパーです。

## 機能

- **シンプルなコマンドライン**: `vp "読み上げるテキスト"`
- **音声プリセット**: 感情とピッチ設定を保存・再利用可能
- **自動テキスト分割**: 140文字を超えるテキストを自然な区切り点で分割
- **自動再生**: 出力ファイルを指定しない場合、mpvで自動再生
- **ファイル入力サポート**: `-t`オプションでファイルからテキストを読み込み
- **包括的な音声制御**: ナレーター、感情、速度、ピッチ設定

## 動作要件

- macOS
- [VOICEPEAK](https://www.ai-j.jp/voicepeak/) が `/Applications/voicepeak.app/` にインストール済み
- [mpv](https://mpv.io/) 音声再生用 (Homebrew経由でインストール: `brew install mpv`)
- [ffmpeg](https://ffmpeg.org/) バッチモードと複数チャンクファイル出力用 (Homebrew経由でインストール: `brew install ffmpeg`)

## インストール

### crates.io から（推奨）

```bash
cargo install voicepeak-cli
```

### ソースから

1. このリポジトリをクローン
2. ビルドしてインストール:
   ```bash
   cargo install --path .
   ```

## 使用方法

### 基本的な使用方法

```bash
# シンプルなテキスト読み上げ（プリセットまたは --narrator が必要）
vp "こんにちは、世界！"

# ナレーターを明示的に指定
vp "こんにちは、世界！" --narrator "夏色花梨"

# 自動再生ではなくファイルに保存
vp "こんにちは、世界！" --narrator "夏色花梨" -o output.wav

# ファイルから読み込み
vp -t input.txt --narrator "夏色花梨"

# パイプ入力
echo "こんにちは、世界！" | vp --narrator "夏色花梨"
cat document.txt | vp -p karin-happy
```

### プリセットの使用

```bash
# 利用可能なプリセットを一覧表示
vp --list-presets

# プリセットを使用
vp "こんにちは、世界！" -p karin-happy

# プリセット設定を上書き
vp "こんにちは、世界！" -p karin-normal --emotion "happy=50"
```

### 音声制御

```bash
# 音声パラメータの制御
vp "こんにちは、世界！" --narrator "夏色花梨" --speed 120 --pitch 50

# 利用可能なナレーターの一覧
vp --list-narrator

# 特定のナレーターの感情一覧
vp --list-emotion "夏色花梨"
```

### テキスト長の処理

```bash
# 自動テキスト分割を許可（デフォルト）
vp "非常に長いテキスト..."

# 厳格モード: 140文字を超えるテキストを拒否
vp "テキスト" --strict-length
```

### 再生モード

```bash
# バッチモード: 全チャンクを生成後、結合して再生（デフォルト）
vp "長いテキスト" --playback-mode batch

# シーケンシャルモード: チャンクを1つずつ生成・再生
vp "長いテキスト" --playback-mode sequential

# 長いテキストのファイル出力（ffmpegでチャンクを結合）
vp "非常に長いテキスト" -o output.wav

# ffmpegなしでのシーケンシャル再生
vp "長いテキスト" --playback-mode sequential
```

## 設定

設定は `~/.config/vp/config.toml` に保存されます。ファイルは初回実行時に自動作成されます。

### 設定例

```toml
default_preset = "karin-custom"

[[presets]]
name = "karin-custom"
narrator = "夏色花梨"
emotions = [
    { name = "hightension", value = 10 },
    { name = "sasayaki", value = 20 },
]
pitch = 30

[[presets]]
name = "karin-normal"
narrator = "夏色花梨"
emotions = []

[[presets]]
name = "karin-happy"
narrator = "夏色花梨"
emotions = [{ name = "hightension", value = 50 }]
```

### 設定フィールド

- `default_preset`: オプション。`-p`オプションが指定されていない場合に使用するプリセット
- `presets`: 音声プリセットの配列

#### プリセットフィールド

- `name`: 一意のプリセット識別子
- `narrator`: 音声ナレーター名
- `emotions`: `name`と`value`を持つ感情パラメータの配列
- `pitch`: オプションのピッチ調整（-300〜300）

## コマンドラインオプション

```
使用方法: vp [OPTIONS] [TEXT]

引数:
  [TEXT]  読み上げるテキスト

オプション:
  -t, --text <FILE>              読み上げるテキストファイル
  -o, --out <FILE>               出力ファイルのパス（オプション - 指定しない場合はmpvで再生）
  -n, --narrator <NAME>          音声の名前
  -e, --emotion <EXPR>           感情表現（例: happy=50,sad=50）
  -p, --preset <NAME>            音声プリセットを使用
      --list-narrator            音声一覧を表示
      --list-emotion <NARRATOR>  指定した音声の感情一覧を表示
      --list-presets             利用可能なプリセットを表示
      --speed <VALUE>            速度（50〜200）
      --pitch <VALUE>            ピッチ（-300〜300）
      --strict-length            140文字を超える入力を拒否（デフォルト: false、分割を許可）
  -h, --help                     ヘルプを表示
  -V, --version                  バージョンを表示
```

## パラメータの優先順位

複数のソースが同じパラメータを指定した場合の優先順位は以下の通りです:

1. コマンドラインオプション（最高優先度）
2. プリセット値
3. デフォルト値 / なし（最低優先度）

例:
- `vp "テキスト" -p my-preset --pitch 100` は pitch=100 を使用（CLI上書き）
- `vp "テキスト" -p my-preset` はプリセットのpitch値を使用
- `vp "テキスト" --narrator "音声"` はピッチ調整なし

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。詳細は[LICENSE](LICENSE)ファイルをご覧ください。

## 貢献

貢献を歓迎します！このプロジェクトに貢献する詳細なガイドラインについては、[CONTRIBUTING.md](CONTRIBUTING.md)をご覧ください。
