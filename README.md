# Code to reproduce bug report: mysql-async 0.29.0 connection issue with TiDB

Bug: mysql-async 0.29.0 (and earlier) do not reuse connections from pool when connecting to TiDB.
The same code works as expected with MariaDB -- see the output below. Note the connection IDs.

Output with TiDB `5.3.0` and MariaDB `10.6.5` on Ubuntu 20.04 (WSL2):
```text
[2022-01-17T08:59:47Z INFO  mysql_async_issue] ===== Starting MariaDB Test =====
[2022-01-17T08:59:47Z INFO  mysql_async_issue] Iteration 0 connection: id=9
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 1 connection: id=9
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 2 connection: id=9
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 3 connection: id=9
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 4 connection: id=9
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 5 connection: id=9
[2022-01-17T08:59:48Z INFO  mysql_async_issue] ===== Starting TiDB Test =====
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 0 connection: id=141
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 1 connection: id=145
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 2 connection: id=149
[2022-01-17T08:59:48Z INFO  mysql_async_issue] Iteration 3 connection: id=153
[2022-01-17T08:59:49Z INFO  mysql_async_issue] Iteration 4 connection: id=157
[2022-01-17T08:59:49Z INFO  mysql_async_issue] Iteration 5 connection: id=161
```


To reproduce:
1. Run both setup sections below
2. If necessary - edit ports in main.rs (if not using 3306/4000 for MariaDB/TiDB respectively)
3. `cargo run`


### MariaDB Setup (baseline - works correctly)

```text
docker pull mariadb:latest
docker run --network host --detach --name mariadb --env MARIADB_USER=user --env MARIADB_PASSWORD=password --env MARIADB_ROOT_PASSWORD=password  mariadb:latest
sudo apt install mysql-client-core-8.0
mysql --comments --host 127.0.0.1 --port 3306 -u root -p
CREATE DATABASE db;
exit
```

### TiDB Setup (does not reuse connections)


```text
curl --proto '=https' --tlsv1.2 -sSf https://tiup-mirrors.pingcap.com/install.sh | sh
source /home/${USER}/.bashrc
tiup playground --db 1 --pd 1 --kv 1 --monitor
sudo apt install mysql-client-core-8.0
mysql --comments --host 127.0.0.1 --port 4000 -u root
CREATE DATABASE db;
exit
```

Note: shell script is from the TiDB quickstart guide: https://docs.pingcap.com/tidb/stable/quick-start-with-tidb



