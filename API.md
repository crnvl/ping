# API Documentation
The officially hosted instance is available under the base domain [ping.qwq.sh](https://ping.qwq.sh). Instances hosted elsewhere may have different base domains, but follow the same API mapping (unless modified of course. If that's the case, please look up the corresponding documentation).

# Message Endpoints
## `GET /posts/:board`
Returns a list of all posts on a specific board. The default board is `all`, which **does not** contain all posts saved in the database. It only contains posts that are not on any other board. The response is a JSON array of objects, each representing a post. The objects have the following properties:
```rs
pub struct Message {
    pub id: String,
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub ref_id: String,
    pub time: String,
}
```
##### Related file: [models.rs:4](./src/models.rs#L4)

## `GET /post/:id`
Returns a single post with the specified id. The response is a JSON object with the following properties:
```rs
pub struct Message {
    pub id: String,
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub ref_id: String,
    pub time: String,
}
```
##### Related file: [models.rs:4](./src/models.rs#L4)

## `GET /post/:id/comments`
Returns a list of all comments on a specific post. The response is a JSON array of messages, each representing a comment. The objects have the following properties:
```rs
pub struct Message {
    pub id: String,
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub ref_id: String,
    pub time: String,
}
```
##### Related file: [models.rs:4](./src/models.rs#L4)

## `POST /posts/:board`
Creates a new post on the specified board. The request body must be a JSON object with the following properties:
```rs
pub struct UserMessage {
    pub thumb_url: Option<String>,
    pub content: String,
    pub username: Option<String>,
    pub ref_id: Option<String>,
}
```
As seen in the `struct`, `thumb_url` and `username` are optional fields. If they are not provided, the server will set the username to "anonymous" and the `thumb_url` to an empty string.
`ref_id` can be used to reference another message id to create a comment.
##### Related file: [models.rs:15](./src/models.rs#L15)

# Board Endpoints
## `GET /boards`
Returns a list of all boards. The response is a JSON array of strings, each representing a board.

