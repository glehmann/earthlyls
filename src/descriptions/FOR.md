## FOR

#### Synopsis

* ```
  FOR [<options>...] <variable-name> IN <expression>
    <for-block>
  END
  ```

#### Description

The `FOR` clause can iterate over the items resulting from the expression `<expression>`. On each iteration, the value of `<variable-name>` is set to the current item in the iteration and the block of commands `<for-block>` is executed in the context of that variable set as a build arg.

The expression may be either a constant list of items (e.g. `foo bar buz`), or the output of a command (e.g. `$(echo foo bar buz)`), or a parameterized list of items (e.g. `foo $BARBUZ`). The result of the expression is then tokenized using the list of separators provided via the `--sep` option. If unspecified, the separator list defaults to `[tab]`, `[new line]` and `[space]` (`\t\n `).

{% hint style='danger' %}
##### Important
Changes to the filesystem in expressions are not preserved. If a file is created as part of a `FOR` expression, then that file will not be present in the build environment for any subsequent commands.
{% endhint %}

#### Examples

As an example, `FOR` may be used to iterate over a list of files for compilation

```Dockerfile
FOR file IN $(ls)
  RUN gcc "${file}" -o "${file}.o" -c
END
```

As another example, `FOR` may be used to iterate over a set of directories in a monorepo and invoking targets within them.

```Dockerfile
FOR dir IN $(ls -d */)
  BUILD "./$dir+build"
END
```

#### Options

##### `--sep <separator-list>`

The list of separators to use when tokenizing the output of the expression. If unspecified, the separator list defaults to `[tab]`, `[new line]` and `[space]` (`\t\n `).

##### `--privileged`

Same as [`RUN --privileged`](#privileged).

##### `--ssh`

Same as [`RUN --ssh`](#ssh).

##### `--no-cache`

Same as [`RUN --no-cache`](#no-cache).

##### `--mount <mount-spec>`

Same as [`RUN --mount <mount-spec>`](#mount-less-than-mount-spec-greater-than).

##### `--secret <env-var>=<secret-id>`

Same as [`RUN --secret <env-var>=<secret-id>`](#secret-less-than-env-var-greater-than-less-than-secret-id-greater-than).

