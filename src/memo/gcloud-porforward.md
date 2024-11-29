gcloud compute ssh では -- の後ろに OpenSSHのオプションを指定できる

```
gcloud compute ssh your-instance-name --through-tunnel-iap -- -N -L 8080:127.0.0.1:8080``
```

port forward したければ

-L: ローカルポートフォワード
-N: リモートでコマンドを実行しない





