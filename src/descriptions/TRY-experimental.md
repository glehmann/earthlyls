## TRY (experimental)

{% hint style='info' %}
##### Note
The `TRY` command is currently incomplete and has experimental status. To use this feature, it must be enabled via `VERSION --try 0.8`.
{% endhint %}

#### Synopsis

* ```
  TRY
    <try-block>
  FINALLY
    <finally-block>
  END
  ```

#### Description

The `TRY` clause executes commands within the `<try-block>`, while ensuring that the `<finally-block>` is always executed, even if the `<try-block>` fails.

This clause is still under active development. For now, only a single `RUN` command is permitted within the `<try-block>`, and only one or more `SAVE ARTIFACT` commands are permitted in the `<finally-block>`. The clause is thus useful for outputting coverage information in unit testing, outputting screenshots in UI integration tests, or outputting `junit.xml`, or similar.

#### Example

```Dockerfile
VERSION --try 0.8

example:
    FROM ...
    TRY
        # only a single RUN command is currently supported
        RUN ./test.sh
    FINALLY
        # only SAVE ARTIFACT commands are supported here
        SAVE ARTIFACT junit.xml AS LOCAL ./
    END
```

