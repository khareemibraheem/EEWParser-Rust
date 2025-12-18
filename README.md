# EEWParser-Rust

EEWParser-Rust は、**気象庁（JMA）が配信する緊急地震速報（EEW）の Telegram 生電文**を解析し、  
**JSON 形式に変換する Rust 製パーサー**です。

ファイル入力および標準入力（stdin）の両方に対応しており、  
サーバー用途・パイプ処理・ログ解析などに利用できます。

---

## 特徴

- 気象庁 EEW Telegram 生電文に対応
- 生電文 → 構造化 JSON への変換
- ファイル入力 / 標準入力（stdin）対応
- Rust による高速・安全な実装
- 正規表現ベースの柔軟な電文解析

---

## プロジェクト構成

```
src/
├── main.rs        # エントリーポイント
├── parser.rs      # EEW 電文デコーダ
├── reader.rs      # ファイル / stdin 読み込み処理
├── regexp.rs      # 正規表現ルール・電文整形
```

### モジュール概要

- **parser**  
  EEW Telegram 電文を解析し、構造化データへ変換します。

- **reader**  
  ファイルまたは標準入力から電文を読み込みます。

- **regexp**  
  改行補完や Telegram フォーマット処理のための正規表現ルールを提供します。

---

## ビルド方法

```bash
cargo build --release
```

生成された実行ファイルは以下に出力されます。

```
target/release/EEWParser-Rust
```

---

## 使い方

### ファイルから電文を解析

```bash
./EEWParser-Rust -p data.txt
```

- `-p <path>` : EEW 生電文を含むファイルを指定

### 標準入力（stdin）から解析

```bash
cat data.txt | ./EEWParser-Rust -s
```

- `-s` : 標準入力から電文を読み込み

---

## 動作仕様

1. 電文をファイルまたは標準入力から読み込み
2. 改行が含まれていない場合、自動的に Telegram 形式へ整形
3. EEW 生電文を解析
4. 結果を JSON として標準出力に出力

---

## 出力例

```json
{
  "CodeType": "Ｍ、最大予測震度及び主要動到達予測時刻の緊急地震速報",
  "Section": "東京",
  "MsgType": "通常",
  "WarnTime": "20240101161418",
  "CommandCode": "11",
  "EqTime": "20240101161008",
  "EqID": "20240101161010",
  "WarnType": "高度利用者向け",
  "WarnCode": "最終",
  "WarnNum": "46",
  "Shinou": "能登半島沖",
  "ShinouLat": 37.6,
  "ShinouLng": 137.2,
  "ShinouDpth": 10,
  "Magnitude": 7.4,
  "Shindo": "7",
  "Rk1": "IPF 法（5 点以上）",
  "Rk2": "IPF 法（5 点以上）",
  "Rk3": "全点全相",
  "Rk4": "",
  "Rk5": "",
  "Rt1": "海域",
  "Rt2": "緊急地震速報（警報）",
  "Rt3": "不明、未設定時、キャンセル時",
  "Rc1": "ほとんど変化なし",
  "Rc2": "不明、未設定時、キャンセル時",
  "Ebis": [
    {
      "Chiiki": "石川県能登",
      "Shindo1": "7",
      "Shindo2": "6強",
      "Time": "//////",
      "Type": "警報",
      "Arrive": "既に到達と予測"
    },
    {
      "Chiiki": "富山県西部",
      "Shindo1": "6弱",
      "Shindo2": "6弱",
      "Time": "161050",
      "Type": "警報",
      "Arrive": "主要動到達時刻の予測なし（PLUM 法による予測）"
    },
    {
      "Chiiki": "石川県加賀",
      "Shindo1": "6弱",
      "Shindo2": "6弱",
      "Time": "161050",
      "Type": "警報",
      "Arrive": "主要動到達時刻の予測なし（PLUM 法による予測）"
    }
  ]
}
```

※ 出力内容は電文の種類・内容により異なります。
