## Testing
1. Start the server with `cargo run`
2. Log in as a User: `curl http://localhost:8000/login -d '{"email": "user@userland.com", "pw": "1234"}' -H 'Content-Type: application/json'`
3. Try to access a user endpoint: `curl http://localhost:8000/user -H 'Authorization: Bearer JWT_TOKEN' -H 'Content-Type: application/json'`


## TODO
current problem: when we try to redirect after a successful login to the settings page (or any page), we are faced with a 404 error in the browser
originating from missing CORS settings. It appears that when we login from the client with its address and then redirecting in the backend, we send 
a request to the server from another localhost address which triggers the CORS settings.
The problem right now is that I am unable to attach the fitting CORS settings when redirecting.

## MISC
- if you want to add more verbose logging with regards to warp, run `RUST_LOG=warp cargo run`



