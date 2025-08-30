# hostoverview

## local dev

```
$ nix develop

$ hostoverview
```

will start the dev server on port 8888. You can access the hostoverview dashboard on http://localhost:8888

## dev dependencies

The hostoverview itself requires some systems data as well as access to a checkout of the deployments and facts repos.

Defaults are set to match the environment of nix-build-p03.

For local dev, you can override the default paths using environment variables, e.g.

```
$ HO_DEPLOYMENTS_PATH=/path/to/deployments/checkout HO_FACTS_PATH=/path/to/facts/checkout hostoverview
```
