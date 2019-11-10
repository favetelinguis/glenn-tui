# glenn-tui
Multi account multi role aws admin tool

## Roadmap
- Setup clients for each client in config.
- Given a search path and a set of clients use `get-parameters-by-path`.
- Integrate with our git-secrets lib to update secrets.
- Poll `describe-services` for a set of clients to monitor deployments 

### Setup
Create a config file `~/.config/glenn/clients.json` with the following structure.

```json
{
  "clients": [
    {
      "key": "<AWS_KEY>",
      "secret": "<AWS_SECRET>",
      "roles": [
        {
          "arn": "<ROLE_ARN>",
          "region": "<AWS_REGION>" 
        }
      ] 
    }
  ] 
}
```

### Usage
To run with logs
`cargo run -- --log 2> error.log`

To run without logs
`cargo run`

Once started switch tabs with `right|left-arrow` quit using `q`.

To install as a binary use `cargo install --path .`, it will then be available as `glenn` given you have added `.cargo/bin/` to the path.