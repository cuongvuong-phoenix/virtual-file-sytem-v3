# Notes

## Ideas

- Either files or folders directly under the same folder must have unique names => Must have one single entity (calle `note`) for both folder and file in the EERD and it must have a `path UNIQUE` column => To distinguish between a folder and a file, add `type` attribute.

- Approach:

  - **Adjacency List:** each node have a pointer pointing to its parent.
    - *Pros:*:
      - Very efficient on `ls` commands.
    - *Cons:*
      - Bad on other commands since it needs to traverse recursively from the root.
  - **Adjacency List with a normalized `path text UNIQUE` column:** same as *Adjacency List* but now each node has an additional column named `path` which store the full path from root to node (`UNIQUE`).
    - *Pros:*
      - Very efficient on `cat`, `ls`, `find` and `rm` commands.
    - *Cons:*
      - Bad on other commands since it still needs to traverse recursively from a specific node, it still better than **Adjacency List**.
  - **Materialized Path with a `path text UNIQUE` :** each node has a `path` column which store the full path from root to node (`PRIMARY KEY`).
    - *Pros:*
      - Very efficient on all commands.
    - *Cons:*
      - `ls`, `up`, `mv` commands will be quite tricky to implement.
      - Some query cannot utilize the *BTree* index because in *Postgres*, `LIKE` command only use *BTree* index when the pattern must consist a static precedence (i.e: `path LIKE /usr/%` work but not with `path LIKE %/usr`)
  - (CHOSEN) **Materialized Path with a `path text[] UNIQUE`:** each node has a `path` column which store the full path from root to node (`PRIMARY KEY`).
    - *Pros:*
      - Very efficient on all commands.
      - Apply *GIN* index easily.
      - Since we are working with *Postgres* array, we can easily utilize some valuable functions such as `array_upper()`,...
    - *Cons:*
      - Some commands still being tricky to implement.

- Should design database in a bottom-up fashion since we can utilize the `CASCADE` behavior of the DBMS.

- **Additional Functional Requirements**:
  - `clear` command: clear the terminal output.
  - `--help` option: print help.

## UI/UX Design

Consists of 2 part:

- *CLI* (core): normal terminal-like UI.
- *Explorer*: a GUI file explorer with ability to create, read, update, delete nodes.

### CLI

- **Inspiration**:
  - [My own alacritty UI with custom OMZ theme](https://github.com/cuongvuong-phoenix/dotfiles).
- **Additional Features**:
  - Support multi instances in the same page (like `tmux` windows).

### Explorer

*Work in process...*

## Issues

### Performance

- [x] *Reading:* enhanced by using `text[]` data type for `path` combined with Postgres GIN indexing.
- *Error handling:*
  - [x] Try to handle common errors such as *Path does not exist* just by checking whether or not the result table contains at least 1 row ==> Decrease the amount of SQL query executions.
  - [x] In complex situations such as the `cr` command, we will need separated SQL check queries. But each will need to be as compact as possible, avoid executing multiple SQL queries in a loop/recursive.
- [ ] *API Response*: the raw data type of `path` is `text[]`, but when communicating between client and server, we will need to join the array into a single string. This will be implemeneted and tested later on.
- [x] *N+1 problem for `ls` command when listing size of folder:* this occurs when `ls` is called in a folder that contains a lot of folder. Since we only need to count the total size of files within each folder, we can directly handle this problem by using `JOIN LATERAL`.

## Concurrency Control

- [x] Since we are using a relational DBMS (PostgreSQL), it has already handled the common **Concurrency problems* via *ACID* principle. Don't need to reinvent the wheel here.
- [x] *Client A is working in a directory but Client B call `rm` command on that*: every commands that Client A call in current working directory will automatically be invalid because it does not exist in the Database.
- [ ] *Client A is working in a directory but Client B call `rm` on that, then call `cr` to create that exact same directory again:* this is where the `id SERIAL PRIMARY KEY` column can be applied.
