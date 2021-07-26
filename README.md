# poller

A simple poll application that is intended to be easy to self-host. This is a personal project that I run at its current version and (lack of) stability, but it may not meet your production standards.

You can see demo polls here:
- Single vote: https://poller-demo.orndorffgrant.com/poll/nCj3LC4rp8
- Multi vote: https://poller-demo.orndorffgrant.com/poll/Pn5UFpE8Xh
- Score vote: https://poller-demo.orndorffgrant.com/poll/rvrB7nOthd

## Build Instructions

```
cd assets-src
npm run build
cd ..
cargo build --release
```

## Running instructions

1. Create a new database file (sqlite)
    ```
    poller init data.db
    ```
    * This will print out an admin user and password
2. Set a secret environment variable (32 random characters)
    ```
    export POLLER_SECRET=asdfasdfasdfasdfasdfasdfasdfasdf
    ```
3. Run poller
    ```
    poller run data.db
    ```
    * This will run on port 8000 (not currently configurable).
4. Set up your reverse proxy that handles TLS to point to poller. I recommend [caddy](https://caddyserver.com/).

## Using poller
1. Log in to poller using the admin user and password that were printed when initializing the database.
2. Create a new non-admin user via the interface.
    * admins cannot create polls - only manage users.
3. Log in as the non-admin user
4. Create a poll via the interface.
5. Send the poll link to anyone you want.
