# Graceful Shutdown on your web application.

## What is a problem?
When you want to deploy your application frequently (e.g. every 5 minutes), it means you shutdown old application regularly.
While shutting down the process, if some requests are aborted, it is an incident. We need to wait for these process to be done. 


## SIGTERM
SIGTERM is a process to "require" "termination, whereas SIGKILL is a process to "force" termination.


In these modern days, most of the cloud service send `SIGTERM` at first when it finishes processes. Then, x seconds after, it sends `SIGKILL`.

| system       |      default behavior            |
|-------------:|---------------------------------:|
| k8s          | SIGTERM -> 30 sec -> SIGKILL     |
| docker stop  | SIGTERM -> 10 sec -> SIGKILL     |
| systemd      | SIGTERM -> 30 sec -> SIGKILL     |
| Amazon ECS   | SIGTERM -> 30 sec -> SIGKILL     | 


## How to implement Grceful Shutdown on your web application.
It's very simple.

### For Synchronous Operations (e.g. General API call)
- 1. Accept SIGTERM
- 2. Disable accepting new HTTP(TCP) connections
- 3. Wait for all processing requests to finish


### For Asynchrnonous Operations (e.g. Background Process)
- 1. Accept SIGTERM
- 2. Disable accepting new asynchronous operations
- 3. Wait for all processing operations to finish

In order to wait for all requests to finish, we just set a grace period between SIGTERM -> SIGKILL to match the exepected maximum execution time of processes.

But, if it's too long, you'll see the phenonmenon that deploy doesn't finish forever ...


## References

- https://link-and-motivation.hatenablog.com/entry/2022/09/29/090000
- https://qiita.com/suin/items/122946f7d0cd13b143f9
