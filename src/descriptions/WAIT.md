## WAIT

#### Synopsis

* ```
  WAIT
    <wait-block>
  END
  ```

#### Description

The `WAIT` clause executes the encapsulated commands and waits for them to complete.
This includes pushing and outputting local artifacts -- a feature which can be used to control the order of interactions with the outside world.

Even though the `WAIT` clause limits parallelism by forcing everything within it to finish executing before continuing, the commands **within** a `WAIT` block execute in parallel.

#### Examples

As an example, a `WAIT` block can be used to build and push to a remote registry (in parallel), then, after that execute a script which requires those images to exist in the remote registry:

```Dockerfile
myimage:
  ...
  SAVE IMAGE --push user/img:tag

myotherimage:
  ...
  SAVE IMAGE --push user/otherimg:tag

WAIT
  BUILD +myimg
  BUILD +myotherimg
END
RUN --push ./deploy ...
```

One can also use a `WAIT` block to control the order in which a `SAVE ARTIFACT ... AS LOCAL` command is executed:

```Dockerfile
RUN ./generate > data
WAIT
  SAVE ARTIFACT data AS LOCAL output/data
END
RUN ./test data # even if this fails, data will have been output
```

