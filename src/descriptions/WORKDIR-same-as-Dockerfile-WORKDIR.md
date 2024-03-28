## WORKDIR (same as Dockerfile WORKDIR)

#### Synopsis

* `WORKDIR <path-to-dir>`

#### Description

The `WORKDIR` command sets the working directory for following commands in the recipe. The working directory is also persisted as the default directory for the image. If the directory does not exist, it is automatically created. This command works the same way as the [Dockerfile `WORKDIR` command](https://docs.docker.com/engine/reference/builder/#workdir).

