# Book API built with Fermyon Spin

This repository contains an HTTP API for managing books. The app is used to explain the new built-in HTTP router provided by Fermyon Spin 1.1.

## Running the Sample

You must have Rust, the `wasm32-wasi` target, and Spin CLI (`v1.1`) installed on your machine.

```bash

# build the app
spin build

# run the app
spin up

##
## alternative approach is to use 
spin build --up
```

## Provided API endpoints

- `GET /` - Return all books
- `GET /:id` - Return a book by its id
- `POST /` - Create a new book
- `PUT /:id` - Update a book by its id
- `DELETE /:id` - Delete a book by its id

Check the `src/models/book.rs` to see actual schema for request payloads and response bodies.


## Further details

To get further details on that sample, consider reading my [corresponding blog post](https://www.thorsten-hans.com/first-glance-at-spin-router-for-rust/).
