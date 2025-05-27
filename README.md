# Require
```
sudo apt update
sudo apt install build-essential cmake ninja-build
```
# How To Use
test以下に各test caseのinput, outputを記述します．
複数の入出力にも対応しています．必ず，inputから始まるテキストファイルと，inputをoutputに置き換えたテキストファイルを用意してください．

```
input_hoge.txt <- OK
output_hoge.txt <- OK
input_huga.txt <- NG output_huga.txtが必要．無視される．
output_piyo.txt <- NG input_piyo.txtが必要．無視される．
foo_input.txt <- NG inputから始まっていない．無視される．
```

```
python3 make_test.py
```
でtest caseからGoogle Testを自動生成します．

```
cmake --preset a
cmake --build --preset a
```
でA問題をビルドできます．他の問題にする場合は適切に`a`を切り替えてください．

```
ctest --preset a
```
でA問題をテストできます．他の問題にする場合は適切に`a`を切り替えてください．