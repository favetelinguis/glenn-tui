# glenn-rs
Multi account multi role aws admin tool

### Setup
Create a config file `config/clients.json` with the following structure.

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
