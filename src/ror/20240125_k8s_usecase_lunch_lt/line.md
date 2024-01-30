k8s は機能拡張されることを想定してつくられているため、プラットフォームエンジニアリングと相性が良い。

LINEでは
- カスタムコントローラー
- Admission Webhookあたりをよくつかう

## カスタムコントローラー
独自フォーマットのmanifestを操作できる
Go言語でcontroller-runtimeというフレームワークをつかって実装する
- クラスタ内になんかしたり
- 外の世界になにかしたり

## LINEヤフーでの活用事例
Webアプリケーションの実行基盤。
簡単なコマンドを実行 or マニフェストを適用するだけでエンドポイントが公開される。

オブザーバビリティ系のサービスにテレメトリを自動送信


## Document
https://speakerdeck.com/hhiroshell/platform-engineering-and-kubernetes-findy-lunch-lt-edition
