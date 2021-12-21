# github-activity
Collect GitHub activities and output by Markdown format

## Setup

1. Generate your personal access token on GitHub. 
    - https://github.com/settings/tokens
2. Sett token to `GITHUB_PERSONAL_ACCESS_TOKEN` environment variable.
    - ```shell
      $ export GITHUB_PERSONAL_ACCESS_TOKEN="(YOUR-GITHUB-PERSONAL-ACCESS-TOKEN)"
      ```

## Development

```shell
$ cargo run --start "2021-12-21T00:00:00+09:00" --end "2021-12-21T23:59:59+09:00"
```
