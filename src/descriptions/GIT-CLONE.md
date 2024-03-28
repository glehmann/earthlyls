## GIT CLONE

#### Synopsis

* `GIT CLONE [--branch <git-ref>] [--keep-ts] <git-url> <dest-path>`

#### Description

The command `GIT CLONE` clones a git repository from `<git-url>`, optionally referenced by `<git-ref>`, into the build environment, within the `<dest-path>`.

In contrast to an operation like `RUN git clone <git-url> <dest-path>`, the command `GIT CLONE` is cache-aware and correctly distinguishes between different git commit IDs when deciding to reuse a previous cache or not. In addition, `GIT CLONE` can also use [Git authentication configuration](../guides/auth.md) passed on to `earthly`, whereas `RUN git clone` would require additional secrets passing, if the repository is not publicly accessible.

Note that the repository is cloned via a shallow-clone opperation (i.e. a single-depth clone).

{% hint style='info' %}

If you need to perform a full-depth clone of a repository, you can use the following pattern:

```Dockerfile
GIT CLONE <git-url> <dest-path>
WORKDIR <dest-path>
ARG git_hash=$(git rev-parse HEAD)
RUN git remote set-url origin <git-url> # only required if using authentication
RUN git fetch --unshallow
```
{% endhint %}

{% hint style='warning' %}
As of Earthly v0.7.21, git credentials are no longer stored in the `.git/config` file; this includes the username.
This means any ssh-based or https-based fetches or pushes will no longer work unless you restore the configured url,
which can be done with:
```Dockerfile
RUN git remote set-url origin <git-url>
```
{% endhint %}

See the "GIT CLONE vs RUN git clone" section under the [best practices guide](../guides/best-practices.md#git-clone-vs-run-git-clone) for more details.

#### Options

##### `--branch <git-ref>`

Points the `HEAD` to the git reference specified by `<git-ref>`. If this option is not specified, then the remote `HEAD` is used instead.

##### `--keep-ts`

Instructs Earthly to not overwrite the file creation timestamps with a constant.

