
# matches マクロ
matches! マクロ
https://qiita.com/niwaka_dev/items/c5b2f5b6587e827a3247

パターンにマッチするかどうかだけを判定するマクロ。

テストで雑に判定したいときに便利。わざわざEqやPartialEqを実装する必要がない。

```rust
assert!(matches!(Some(42), Some(42)));
```
