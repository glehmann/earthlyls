## ENTRYPOINT (same as Dockerfile ENTRYPOINT)

#### Synopsis

* `ENTRYPOINT ["executable", "arg1", "arg2"]` (exec form)
* `ENTRYPOINT command arg1 arg2` (shell form)

#### Description

The `ENTRYPOINT` command sets the default command or executable to be run when the image is executed as a container. It works the same way as the [Dockerfile `ENTRYPOINT` command](https://docs.docker.com/engine/reference/builder/#entrypoint).

