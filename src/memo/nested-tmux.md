tmux-session の 中で tmux を使いたいときがある。
たとえばローカルのtmuxでリモートサーバーにsshして、その中で更にtmuxを使いたいときなど。

そういうときは prefix key を使うのがよい

設定はこれだけ。Ctrl-a で tmux-session内でもprefixを送れるようにする。
```tmux.conf
bind-key a send-prefix
```

Nestしてtmuxを使いたいときは以下のようにすればよい
```
Ctrl-b a
```

https://blog.ccm-lulu.com/2013/02/tmux-nested-tmux-tmux-tmux.html
https://mutelight.org/practical-tmux
