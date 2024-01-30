# zozo: k8sを用いた開発環境
山岡さん @ymkmtk

## 負荷試験基盤
- k6を負荷試験ツールとして統一
- k8s で 複数podで負荷試験

githubに負荷試験をpush
github actionで負荷試験を自動実行
podが立って負荷試験が実行される
負荷試験結果はDataDogにExportされる。

## Pull Request毎のPreview環境
フロントエンドのリプレイスで旧環境のUIと比較しながらプレビューしたい。

- pr100.preview.wear.jp にアクセスするとPR100のPreview環境へ
  - ワイルドカードドメインとする (*.preview.wear.jp)
  - Istio VirtualService の ホストアドレスルーティングで実現
- Pull Requestを検知してk8sへデプロイ
  - Argo CD Pull Request Generator
  - Helm Template で VirtualSrvice, Deployment を管理

## 今年は
2022 GitOps化
2023 負荷試験環境、Preview環境
2024 QA環境

## Refs
https://techblog.zozo.com/entry/wear-kubernetes-load-test-platform
https://speakerdeck.com/ymktmk/wearhurontoendoniokerupull-requestmei-nopreviewhuan-jing-dao-ru-tosonoxiao-guo
https://techblog.zozo.com/entry/reconfigure-eks-workflow-infrastructure
