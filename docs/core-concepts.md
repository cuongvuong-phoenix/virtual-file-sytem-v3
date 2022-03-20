# Core Concepts

Core concepts to implement the *Virtual File System* based on [the requirements version 3](./requirements.md)

## Single entity for both folder and file

Based on constraint [C-4](./requirements.md#c-4), we need a single entity for both folder and file to utilize `UNIQUE` constraint.

To distinguish between the two, we then need a `is_folder BOOLEAN` column.

The single entity will be named as `node`.

## Modeling the system

A file/folder structure can be categorized as a hierarchical data structure. Therefore, to implement it, we need to find a good database model to best-fit the requirements. First is the concept.

### Hierarchical database model

Based on [the wiki](https://en.wikipedia.org/wiki/Hierarchical_database_model), a *hirarchical database model* is a data model in which the data are organized into a tree-like stucture.

In the sense of database system, data are stored as *records* (same as nodes of tree). To connect one record to another, the DBMS use foreign key constraints. However, unlike traditional implementations of tree structure that each node holds a list of its children, every record in a database can only reference to one other record. Although [most DBMSs support array type](https://www.postgresql.org/docs/current/arrays.html), yet it defeats many aspects that only happen in the database system. For example, to update a node in a tree, the SQL query need to be executed will be quite tricky to mimic the behavior like a trivial programming language, and may also include multiple queries which is considered inefficient.

There are many ways/patterns to implement a hierarchical database model including: *Adjacency List*, *Nested Set*, *Nested Interval*, *Closure Table*, *Materialized Path*,... Each also consists of one/more variants based on the application requirements. For this particular *Virtual File System v3* requirements, I will only describe **4 models** that I think worth to mention:

- *Adjacency List*.
- *Adjacency List with a `path TEXT` column*.
- *Materialized Path with a `path TEXT` column*.
- **(CHOSEN)** *Materialized Path with a `path TEXT[]` column*.

### Adjacency List

This is the simplest model and is very easy to understand: the child record stores a reference to its parent.

Because the reference is implemented using foreign key constraint, it always maintain referential integrity.

The schema for this model is as follow:

```sql
CREATE TABLE node (
    id SERIAL PRIMARY KEY,
    name TEXT, -- NULL for root
    is_folder BOOLEAN NOT NULL,
    data TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    
    parent_id REFERENCES node (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT valid_name CHECK ("name" ~ '^[a-zA-Z0-9 _-]+$')
);
```

From the schema, we can tell that it does not guarantee constraint [C-4](./requirements.md#c-4), because the `name` column cannot be `UNIQUE` to accept same file/folder name but in different folders.

However, let's assume our application never violates constraint [C-4](./requirements.md#c-4), this model will provide us following pros and cons:

- *Pros:*
  - Very efficient on `cr` command.
  - Utilize the *Postgres Recursive CTEs (Common Table Expressions)* to improve the traversing performance rapidly over *Nested Set* model ([benchmark](https://explainextended.com/2009/09/24/adjacency-list-vs-nested-sets-postgresql/)).
- *Cons:*
  - Almost every command that need to fetch/find a single node in the table is expensive because we need to traverse recursively from the root node to reach it.
  - Although in theory, insert, update, move and delete operations are very efficient (thanks to `CASCADE`), it only applies when we know the exact `id` of the node from the start. Meanwhile, in our application, the client only provides the full path, therefore we will need to traverse from the root node to build up the path and find the correct node to manipulate, and this is an expensive process.

From above analyses, we can confidently avoid using this model to implement our system. However, this is still a good fundamental lead to the next model: *Adjacency List with `path TEXT` column*.

### Adjacency List with a `path TEXT` column

The concept is same as *Adjacency List* model excepts that now we have an additional `path TEXT` column which store the full path of each node.

To ensure constraint [C-4](./requirements.md#c-4), we could easily add `UNIQUE` constraint on the `path` column.

The schema for this model is as follow:

```sql
CREATE TABLE node (
    id SERIAL PRIMARY KEY,
    "path" TEXT UNIQUE, -- NULL for root
    "name" TEXT, -- NULL for root
    is_folder BOOLEAN NOT NULL,
    "data" TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    parent_id REFERENCES node (id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT valid_name CHECK ("name" ~ '^[a-zA-Z0-9 _-]+$')
);
```

Based on the schema, we can analyze some pros and cons:

- *Pros:*
  - Inherit the base *Adjacency List* model with a (huge) fix on fetching/finding problem, thus many commands are efficient now including: `cd`, `cr`, `cat`, `ls` and `find`.

- *Cons:*
  - `up`, `mv` and `rm` commands still need to traverse recursively from a specific node to ensure data and referential integrity.

This is the first model I used to implement the file system logic. However, the SQL queries for some commands are very tricky, hard to read and I doubt the performance of Recursive CTEs over following models using *Materialized Path* pattern.

### Materialized Path with a `path TEXT` column

The concept of *Materialized Path* is also quite simple: each node store a full path of it in the table.

It is like the *Adjacency List with a `path TEXT` column* model above but it's not: we do not store a parent reference in each node anymore. And as a result, this may not ensure referential integrity for our system and may cause orphan nodes which users can not access using relative paths (paths relative to the current working directory).

However, this problem can be solved by simple checks on insert and update operations or by strictly preventing in the application layer.

The schema for this model is as follow:

```sql
CREATE TABLE node (
    id SERIAL PRIMARY KEY,
    "path" TEXT UNIQUE, -- NULL for root
    "name" TEXT, -- NULL for root
    is_folder BOOLEAN NOT NULL,
    "data" TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    CONSTRAINT valid_name CHECK ("name" ~ '^[a-zA-Z0-9 _-]+$')
)
```

- *Pros:*
  - Very efficient on fetching/finding commands.
  - A little bit less efficient for updating commands, but we can utilize *BTree* index to only have a fast flat-lookup instead of recursively traversing like *Adjacency List* models.
- *Cons:*
  - Not guarantee referential integrity (and also its solution) as described above.
  - Working with raw string in database is not easy, and the SQL queries for some commands (`ls`, `up`, `mv`) will be hard to read and reason about.
  - Some queries cannot utilize the *BTree* index because in *Postgres*, `LIKE` command only use *BTree* index when the pattern consists a static precedence (i.e: `path LIKE /usr/%` works but not with `path LIKE %/usr`)

This model is almost ideal for our application and I've also experienced and tested it. However, I quickly realize I can easily implement a *Materialized Path*-based model by declaring `path` column as a `TEXT[]` type and I eventually chose it as the model to implement the system.

### (CHOSEN) Materialized Path with a `path TEXT[]` column

This is the same as above model, excepts that it use an array of strings to store the full path instead of a single string.

Actually, the standard type for `path` is `int[]`, and it should store an array of `id`s, starts from the root node. However, if we use this standard pattern, it would be pointless, since the client only provides the full path as a raw string, we have no way to map a raw string to an array of `id`s without recursively traversing from the root node.

The schema for this model is as follow:

```sql
CREATE TABLE node (
    id SERIAL PRIMARY KEY,
    "path" TEXT[] UNIQUE NOT NULL,
    is_folder BOOLEAN NOT NULL,
    "data" TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    CONSTRAINT valid_name CHECK ("path"[array_upper("path", 1)] ~  '^[a-zA-Z0-9 _-]+$')
);
```

This schema provides us alot of pros and also decrease the cons of the above models:

- *Pros:*
  - Very efficient on every command.
  - Easily apply *GIN* index on the `path` column for every command.
  - Better readability since we can use some array functions when implementing the logic.
- *Cons:*
  - Some commands are still tricky.

In my opinion, this is the ideal model that best-fit the requirements, therefore I chose it to implement the system.

## Resolving path aliases

Based on chosen database model, the path aliases resoltion should be processed in the application layer.

The strategy for this is simple: we use a single loop over path segments, if a segment equals one of defined aliases, we correctly handle that.

Besides, to increase performance, this process should also be ran on the browser to avoid server overhead.

## Parsing raw string to CLI arguments

Users use the application via a browser environment, and the full command that user typed is a raw string, therefore, we need to parse it into correct arguments defined in the requirements.

On the other hand, the constraint [C-3](./requirements.md#c-3) says any path segment can include whitespace character, thus, we cannot use the simple `split(' ')` function to parse the raw string.

According to [this issue](https://github.com/nodejs/help/issues/1218), the *Node.JS* `process.env.argv` implementation seems to be really complicated. Therefore, I will not try to "reinvent the wheel" at this moment, and instead, using the `yargs` library with a browser-compatible javascript module. This library also provides basic user manual via `--help` option which is very useful and convenience.
