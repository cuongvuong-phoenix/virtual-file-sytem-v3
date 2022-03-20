# Requirements

Requirements version 3 for *Virtual File System*.

## Concepts

- File system: a system to manage files. It consists of files and folders
- Folder (or directory): a group of file(s) and folder(s).
  - Attributes:
    - `name`: name of the folder
    - `created_at`: the timestamp when the folder was created
- File: an entity that stores data
  - Attributes:
    - `name`: name of the file
    - `data`: the data that the file holds. Let's say we only support text (not binary) data
    - `created_at`: the timestamp when the file was created

## Constraints

### C-1

A folder can contain both files and folders

### C-2

A folder or file can only reside in 1 place, i.e. in 1 folder/path at a time

### C-3

A valid folder/file name is a string that matches the following regex: `/^[a-zA-Z0-9 _-]+$/` (note that the space character is allowed)

### C-4

Items (either files or folders) directly under the same folder must have unique names

## Functional

1. A command line interface on the web page, which supports the following commands (that should work very similar to the commands on UNIX systems):
    1. `cd FOLDER_PATH`: change current working directory/folder to the specified `FOLDER`
    2. `cr [-p] PATH [DATA]`: create a new file (if `DATA` is specified, otherwise create a new folder) at the specified `PATH`
        1. if the parent folder of the destination `PATH` does not exist:
            1. **Bonus feature:** if optional param `-p` is specified, create the missing parent folders
            2. else, raise error
        2. if there is an existing file/folder at `PATH`, raise error
    3. `cat FILE_PATH`: show the content of a file at `FILE_PATH`. If there is no file at `FILE_PATH`, raise error.
    4. `ls [FOLDER_PATH]`: list out all items **directly under** a folder
        1. the output list must include name, created_at, and size of each item **directly under** the current folder, and of the current folder itself. Size of a folder is the total size of all files within the folder. Size of a file is the number of characters in its data.
        2. if the optional param `FOLDER_PATH` is specified, list items in the folder at `FOLDER_PATH`. Otherwise if omitted, list items in the current working folder
    5. `find NAME [FOLDER_PATH]`: search all files/folders whose name **contains** the substring `NAME`. If the optional param `FOLDER_PATH` is specified, find in the folder at `FOLDER_PATH`. Otherwise if omitted, find in the current working folder. Note:
        1. the command should find in subfolders as well
        2. the result should be displayed nicely to end users
    6. `up PATH NAME [DATA]` update the file/folder at `PATH` to have new `NAME` and, optionally, new `DATA`
    7. `mv PATH FOLDER_PATH` move a file/folder at `PATH` **into** the destination `FOLDER_PATH`. Raise error if:
        1. there is no Folder at `FOLDER_PATH`
        2. `FOLDER_PATH` is sub-path of `PATH`. In other words, cannot move a folder to become a subfolder of itself
    8. `rm PATH [PATH2 PATH3...]`: remove files/folders at the specified `PATH`(s)
2. The commands must support these aliases:
    1. `.` ****(dot): current working folder
    2. `..` ****(double dots): parent folder
    3. `/` ****(slash): root folder
3. There should be a user manual that users can easily access

## Non-functional

- The operations must be efficient. For example, `ls` should not fetch any records other than the target folder and its direct children from the database
- There should be no orphan item in the database, meaning all items should be accessible by the user
- *Performance*: the system should be able to handle a large number of items and still deliver good user experience
- *Security*: the users should only be able to access and manipulate the virtual file system, nothing more on your server
- *Concurrency management*: your app should be able to adequately handle multiple users accessing and modifying the file system at the same time. Please list out some concurrency issues that may happen with your app, and how you choose to handle them.
