# Specifics that we want to reuse from Git

## How many characters are needed to identify an ObjectID

```bash
$ git log --oneline 7130
# good output
$ git log --oneline 713
fatal: ambiguous argument '713': unknown revision or path not in the working tree.
Use '--' to separate paths from revisions, like this:
'git <command> [<revision>...] -- [<file>...]'
```
