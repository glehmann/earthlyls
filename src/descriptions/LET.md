## LET

#### Synopsis

* `LET <name>=<value>`

#### Description

The command `LET` declares a variable with the name `<name>` and with a value `<value>`. This command works similarly to `ARG` except that it cannot be overridden.

`LET` variables are allowed to shadow `ARG` build arguments, which allows you to promote an `ARG` to a local variable so that it may be used with `SET`.

##### Example

```
VERSION 0.8

# mode defines the build mode. Valid values are 'dev' and 'prod'.
ARG --global mode = dev

foo:
    LET buildArgs = --mode development
    IF [ "$mode" = "prod" ]
        SET buildArgs = --mode production --optimize
    END
```

