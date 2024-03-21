## Testing
1. Start the server with `cargo run`
2. Log in as a User: `curl http://localhost:8000/login -d '{"email": "user@userland.com", "pw": "1234"}' -H 'Content-Type: application/json'`
3. Try to access a user endpoint: `curl http://localhost:8000/user -H 'Authorization: Bearer JWT_TOKEN' -H 'Content-Type: application/json'`


## TODO