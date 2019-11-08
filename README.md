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

To enable logging set env:
RUST_LOG=debug
