## TRIGGER (**deprecated**)

{% hint style='info' %}
##### Note
The `TRIGGER` command is in beta status and is only useful for Earthly CI.
{% endhint %}

#### Synopsis

* `TRIGGER manual` (manual form)
* `TRIGGER pr <pr-branch>` (PR form)
* `TRIGGER push <push-branch>` (push form)

#### Description

The `TRIGGER` command is only allowed in the context of a pipeline target (declared via `PIPELINE`), and is used to configure the way in which the pipeline is triggered. Multiple triggers are allowed for a single pipeline.

In the *manual form*, the pipeline is triggered manually via the Earthly CI UI or via `earthly` on the command-line.

In the *PR form*, the pipeline is triggered when a pull request is opened against the branch `<pr-branch>`.

In the *push form*, the pipeline is triggered when a push is made to the branch `<push-branch>`.

{% hint style='info' %}
##### Note
Pipelines and their definitions, including their triggers must be merged into the primary branch (which, unless overridden, is the default branch on GitHub) in order for the triggers to take effect.
{% endhint %}

