# Contribution Guide

## Guidelines

* All changes ideally must be first suggested in an issue and then contributed via Pull Requests.
* If your proposed change warrants a test, please add one.
* If you're changing API behaviour, please update the docstrings.
* Please have a look at the various linters being used and make sure to run them periodically and **especially** before pushing.
* Every PR must pass all tests in CI.

## Advice for the developer

* We have a `.pre-commit-config.yaml` file that you can use for general code sanitization.
* You can use the following zed configuration file to set up your development environment:
```JSON
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "features": ["image-to-ascii", "image"]
        }
      }
    }
  }
}
```
