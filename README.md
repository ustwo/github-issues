# Github Issues CLI

The current version implements a `fetch` command to get all issues for a
Github repository and it stores as CSV.


## In short

```sh
github-issues fetch ustwo/ustwo.com-frontend \
                    --oauth-token=$GITHUB_TOKEN \
                    --format=csv \
                    --output=usweb.csv \
                    --label=in_backlog \
                    --label=bug
```


## Install

The preferred way to install `github-issues` is via Hombrew:

```sh
brew install ustwo/tools/github-issues
```

Alternatively, grab a [release](https://github.com/ustwo/github-issues/releases)
and place it in your `$PATH`.


## Contributing

Check our [contributing guidelines](./CONTRIBUTING.md)


## Maintainers

* [Arnau Siches](mailto:arnau@ustwo.com)


## Contact

open.source@ustwo.com


## License

This is a proof of concept with no guarantee of active maintenance.

Licensed under [MIT](./LICENSE).
