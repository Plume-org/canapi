# Canapi

Common Canapi traits.

Canapi is a collection of Rust crate to make it possible to share a REST API definition between
clients and servers. You define your endpoints (`Endpoint` trait), bind them to your databse (`Provider` trait),
and you can re-use these endpoints in your client (`Fetch` trait).

This requires three separate crates: one for your API defintion, one for your server, and one for the client.

## Example

`my-api`, where you define the API.

```rust
use canapi::Endpoint;

#[derive(Default, Serialize, Deserialize)]
struct UserEndpoint {
    id: Option<i32>,
    name: Option<String>,
    bio: Option<String>,
}

impl Endpoint for PostEndpoint {
    type Id = i32;

    fn endpoint() -> &'static str {
        "/api/v1/users"
    }
}
```

`my-server`, to bind endpoints to database (and expose them to the rest of the world)

```rust
use canapi::Provider;
use diesel::*; // Example with Diesel
use my_api::UserEndpoint;

/// Define the User model…

impl Provider<PgConnection> for User {
    type Data = UserEndpoint;

    fn get(conn: &PgConnection, id: i32) -> Option<Post> {
        posts::table.filter(posts::id.eq(id))
            .limit(1)
            .get_result::<Post>(conn)
            .ok()
    }
}

// Use rocket, actix-web, iron, gotham or whatever you want to expose your endpoints…
```

`my-client`, to use your API (here with a potential WASM `Fetch` impl)

```rust
use canapi_wasm::WasmFetch;
use plume_api;

#[derive(Default)]
pub struct Api {
    users: UserEndpoint
}

fn fetch_first_post() {
    let api = Api::default();
    let post = api.users.get::<WasmFetch>(1);
    assert_eq(post.id, Some(1));
}

fn find_hello() {
    let api = Api::default();
    let post = api.users.find::<WasmFetch>(PostEndpoint {
        name: Some(String::new("Jane")),
        ..PostEndpoint::default()
    });
    assert_eq!(post.title, Some(String::new("Jane")));
}
```
