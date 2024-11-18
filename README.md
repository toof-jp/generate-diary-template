# diary-template-generator
[![Actions Status](https://github.com/toof-jp/diary-template-generator/workflows/Rust/badge.svg)](https://github.com/toof-jp/diary-template-generator/actions)

1週間分の日記のMarkdownのテンプレートを生成するためのコマンドラインツール  
実行すると現在の週の月曜日からの1週間分の日記のテンプレートが生成される

## Usage
```
$ date
Fri Aug  4 03:45:57 PM JST 2023
$ cargo run --quiet
# 2023-07-31
## 2023-07-31

## 2023-08-01

## 2023-08-02

## 2023-08-03

## 2023-08-04

## 2023-08-05

## 2023-08-06

```

## Related
[diary-template-generator-frontend](https://github.com/toof-jp/diary-template-generator-frontend)
