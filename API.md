# API Documentation
The officially hosted instance is available under the base domain [ping.qwq.sh](https://ping.qwq.sh). Instances hosted elsewhere may have different base domains, but follow the same API mapping (unless modified of course. If that's the case, please look up the corresponding documentation).

## `GET /posts/:board`
Returns a list of all posts on a specific board. The default board is `all`, which **does not** contain all posts saved in the database. It only contains posts that are not on any other board. The response is a JSON array of objects, each representing a post. The objects have the following properties:
```rs
pub struct Message {
    pub board: String,
    pub thumb_url: String,
    pub content: String,
    pub username: String,
    pub time: String,
}
```
##### Related file: [models.rs:4](./src/models.rs#L4)