# Github Issues CLI

The current version allows you to fetch or upload Github issues. `fetch` lets
you store the retrieved issues in CSV or JSON. `upload` lets you create
multiple issues from a CSV.


## In short

### Fetch issues

Get all issues from `ustwo/ustwo.com-frontend` labelled as `in_backlog` or
`bug` and store them as CSV in `ustwo.csv`.

```sh
github-issues fetch ustwo/ustwo.com-frontend \
                    --oauth-token=$GITHUB_TOKEN \
                    --format=csv \
                    --output=usweb.csv \
                    --label=in_backlog \
                    --label=bug
```


### Upload issues

Create a file `myissues.csv` with the following format:

```csv
title,body,labels,assignees,milestone_id
"A nice title","A descriptive body","in_backlog,feature",arnau,1
"Another issue","foo bar","chore",,
```

And run

```sh
github-issues upload ustwo/ustwo.com-frontend \
                     --oauth-token=$GITHUB_TOKEN \
                     --input=myissues.csv
```

The order of the fields is fixed: `title`, `body`, `labels`, `assignees`,
`milestone_id`. And `title` is the only required field so the minimum record
possible is:

```csv
A title,,,,
```

Note: Github allows you to create labels by just setting them in a new Issue
but it will fail if you reference a non-existing milestone id.

The output in the screen will be showing the progress like this:

```
Info: Created issue number 18 A nice title
Info: Created issue number 19 Another issue
```

And it will reflect an error like:

```
Error: Couldn't create an issue for 'Foo bar' because the field 'milestone' has an invalid value.
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
