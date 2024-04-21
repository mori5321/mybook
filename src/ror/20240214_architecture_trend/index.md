# Backend
## ヘキサゴナルアーキテクチャ
ビジネスを中心に見立て、それ以外を交換可能(Plugable)にする
別名 Port and Adapter

既知の内容が多かったため割愛。

# Frontend (GUI Pattern)
## Classic MVC
Small Talk とかの時代。WebのためのArchitectureではない。
だいぶ低レイヤなことをやっていた。
ex: ユーザーがクリックします。この座標にあるオブジェクトはなんですか。

## MVC 2
Web 上で Classic MVC を ガッチャンコしたらどうなりますか。
JSP2.0の時代。


## MVP (nrslibさんはこれが好きらしい)
ビューをウィジェットとして扱う。ビューとコントローラーの責務を

### Observe View?
User
  |
View ---> Presenter
  ^            |
  | observe    v
  --------- Model 


### Passive View

User
  |
View ---> Presenter
     <---      |
               |
               v
             Model 


### MVVM
React, Vue, Angular


### MVW (M, V and Whatever)



# その他
## マイクロサービスアーキテクチャ
- 有識者定義「各サービスがHTTPで通信する」
- nrslib的定義「データストアをわけること」

## モジュラモノリス
- モジュール間の連携で開発者の思慮分別が必要(妙な依存関係をつくるなよ)

## クラウドネイティブ
### 主な構成要素
- コンテナ
- マイクロサービス
- サービスメッシュ
- 宣言型
- イミュータブルインフラストラクチャ

### 達成したいこと
- スケーラビリティ
- 運用コスト削減
- コスト削減

### なぜスケーラビリティを確保したいのか? 
=> システムになるたけ早く応答させたい。
=> それはスケーラビリティだけで確保できるのか...? 

### リアクティブ宣言
即応性、弾力性、メッセージ駆動、耐障害性
=> アプリの形も変えないと、結局クラウドネイティブにならない...。



# CQRS + ES (nrslib トレンド)
Command Query Responsibility Segregation. 
求められる要件が全然違うんだからさぁ。

Query は パフォーマンス重要。


コマンドスタックのデータ。
イベントデータをそのまま保存する(JSON)。

コマンドのデータをRead側のデータに変換するプロセスがある。(ReadModelUpdater, Projector)

Kafkaでやってる。

## Pros
- インピーダンスミスマッチを避けられる。
- コマンドとクエリの異なる要件に答えられる。

## Cons
伝統的な実装よりは複雑
システムの数が増える。

## Pub/Sub
- リアクティブネス

