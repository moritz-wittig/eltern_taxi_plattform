## Testing
1. Change directory to frontend `cd frontend`
2. Start Frontend trunk server: `trunk serve --open`
3. Enter user `user@userland.com` with pw `1234` into the login page
4. When entering the browser console, you can check the logs
5. Enter any other email -> check the logs (fail)


## Random
- after successful login, JWT is stored locally in the browser in the SessionStorage.
This can be checked by going to the Browser Developer Tools -> Application -> Storage
--> SessionStorage --> Key: JWT

## TODO
- currently there seems to be again issues with CORS settings
- further the next idea would be to add a new page where we then 
use the stored JWT and authorize with that.