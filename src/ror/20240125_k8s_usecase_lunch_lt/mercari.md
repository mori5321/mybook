# 人間によるk8sリソース最適化の諦め
メルカリ 中田さん

## Kubernetes in Mercari
- 少ないCluster(2~3)上ですべてのWorkloadが動いている
- 1000+ Deployment
- PlatfomチームがCluster adminとして運用
  - Platformチームが抽象化, CI/CDも提供することで利用ハードルを下げている

## リソース使用率改善の取り組み
リソース最適化には k8s の深い知識が必要。
しかしその知識を全アプリケーション開発者に求めるのは現実的ではない。
=> Platform Teamがリソース最適化のためのツールやガイドラインを提供

## Tool1: Resource Recommender
Slack bot. 
おすすめのCPU, Memory推奨値をポストする。

課題
- 適応してくれない
- 適応してくれたかわからない
- 推奨値が結構頻繁に変わる(リソース使用量に基づいて計算しているため、ビジネス状況によってコロコロ変わりうる)

## AutoScaler
Horizontal Pod Auto Scaler => Scale Out
Vertical Pod Auto Scaler => Resource Request を増減

メルカリではHPAが人気

HPAを最適化しない限りCPU Usageが最適にはならない。
HPAの最適化のために Requestを調整する必要がある場合があり難しい。
サービスが動き続ける限り、最適なパラメータも変化しつづける。
終わりのない戦い。

=> 無理じゃね...?

## リクガメを利用したAutoScaling
mercari/tortoise

過去のWorkloadの振る舞いを記録し、Podの数、resource request limit をすべていい感じに調整してくれる。

## What is good point of リクガメ
ユーザー目線
- リソースの設定/最適化から完全に逃れることができる

リソース最適化の責務を完全にPlatformチームに移動することに成功した。

## インターフェイスもめっちゃシンプル

## Productionではまだつかっていない。
開発環境で試用中
