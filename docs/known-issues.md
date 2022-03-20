# Known issues

## Performance

- [x] *Reading:* enhanced by using `text[]` data type for `path` combined with *Postgres GIN* indexing.
- *Error handling:*
  - [x] Try to handle common errors such as *Path does not exist* just by checking whether or not the result table contains at least 1 row ==> Decrease the needed amount of SQL query executions.
  - [x] In complex situations such as the `cr` command, we will need multi-separated SQL check queries. But each will need to be as compact as possible, avoid executing multiple SQL queries in a loop/recursive.
- [ ] *API Response*: the raw data type of `path` is `text[]`, but when communicating between client and server, we will need to join the array into a single string. (This will be implemeneted and tested later on)
- [x] *N+1 problem for `ls` command when listing size of folders:* this occurs when `ls` is called in a folder that contains a lot of folder. Since we only need to count the total size of files within each folder, we can directly handle this problem by using `JOIN LATERAL`.

## Concurrency Control

- [x] Since we are using a relational DBMS (*PostgreSQL*), it has already handled the common **Concurrency problems* via *ACID* principles.
- [x] *Client A is working in a directory but Client B call `up`/`mv`/`rm` command on it*: every commands that Client A call using current working directory will automatically be invalid because it does not exist in the Database.
- [ ] *Client A is working in a directory but Client B call `rm` on it, then call `cr` to create that exact same directory again:* this is where the `id SERIAL PRIMARY KEY` column can be applied.
