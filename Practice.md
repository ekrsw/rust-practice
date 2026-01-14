了解です。**Lv1→Lv2相当**で作れる、標準ライブラリ中心の **CLIミニ課題**を10個出します（小さく作って達成感が出るやつ）。

---

# Rust CLIミニ課題 10本（Lv1→Lv2）

## 1) echo（引数をそのまま出す）

**仕様**

* `my_echo hello world` → `hello world` を出力（スペース区切りで結合）
* 引数が無い場合 → `Usage: my_echo <text...>`

**狙い**：`std::env::args()`、Vec、join

---

## 2) greet（名前が無ければ標準入力）

**仕様**

* `greet Alice` → `Hello, Alice!`
* 引数が無ければ標準入力で1行読み取り、その名前で挨拶

**狙い**：`stdin().read_line`、分岐、trim

---

## 3) sum（数値引数の合計）

**仕様**

* `sum 10 20 3` → `33`
* 数値に変換できない引数があれば `Error: not a number: xxx` で終了（非0終了コードだと尚良）

**狙い**：`parse::<i32>()`、`Result`、エラーメッセージ

---

## 4) wc-lite（行数・単語数・バイト数）

**仕様**

* 引数にファイルパス：`wc file.txt`
* 出力例：`lines=12 words=98 bytes=1024`
* 引数なしなら標準入力を対象にする

**狙い**：`std::fs::read_to_string` / `read_to_end`、split_whitespace

---

## 5) head（先頭N行だけ表示）

**仕様**

* `head file.txt` は先頭10行
* `head -n 5 file.txt` は先頭5行
* 行が少ない場合はある分だけ表示

**狙い**：簡単なオプション解析、イテレータ、`lines()`

---

## 6) tail（末尾N行だけ表示）

**仕様**

* `tail file.txt` は末尾10行
* `tail -n 20 file.txt` は末尾20行
* メモリ節約は気にしなくてOK（Lv2）

**狙い**：Vecに行を溜める、スライス、`max(0, len-n)`

---

## 7) grep-lite（部分一致検索）

**仕様**

* `grep keyword file.txt`：`keyword` を含む行だけ表示
* `-n` を付けたら行番号も表示：`grep -n key file.txt` → `12: ...`

**狙い**：文字列検索 `contains`、列挙 `enumerate()`

---

## 8) replace（ファイル内の文字列置換して出力）

**仕様**

* `replace from to file.txt` → 置換後を標準出力へ
* `--inplace` があればファイルを上書き保存

**狙い**：`replace`、ファイル書き込み、オプション分岐

---

## 9) calc（簡易電卓：a op b）

**仕様**

* `calc 10 + 3` → `13`
* `calc 10 / 0` → `Error: divide by zero`
* 演算子は `+ - * /` のみ（整数）

**狙い**：match、エラー処理、引数の形をチェック

---

## 10) todo（超ミニToDo：追記＆一覧）

**仕様**

* 保存先は `todo.txt`（カレントディレクトリ）
* `todo add "buy milk"`：1行追記
* `todo list`：行番号付きで一覧表示
* それ以外はUsage表示

**狙い**：ファイル追記 `OpenOptions::new().append(true)`、簡単なサブコマンド

---

## 進め方のおすすめ

* まず **1→3**（args/parse/Result）でCLIの骨格に慣れる
* 次に **4→7**（ファイル＆文字列処理）
* 余力があれば **8〜10**（書き込み・状態保持）

---

やるならまずどれからいきます？
迷うなら **(3) sum** を題材にして、テンプレ（引数パース＋エラーハンドリング）付きで一緒に実装できます。
