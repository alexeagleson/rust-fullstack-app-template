# What is this?

This project is a template for a fullstack web app using [Axum](https://github.com/tokio-rs/axum) as the backend in Rust, and React with [Vite](https://vitejs.dev/) for the frontend.

It is intended as a teaching tool, a complete copy of the blog post that it was created for is included further down in this README.

# How to Use

You must have both [Rust](https://www.rust-lang.org/) and [Node.js](https://nodejs.org/en/) installed.

First boot up the Rust web server, then start the React client.

To run the server:

```
cargo run
```

To run the client:

```
cd client
npm run dev
```

Once both are running you can access the app on [http://localhost:5173/]()

The complete tutorial on how this project was built follows below:

1. [Motivation](#motivation)
1. [Project Setup](#project-setup)
1. [Rust Server](#rust-server)
1. [React Client](#react-client)
1. [Add Styles](#add-styles)
1. [Conclusion](#conclusion)

# Motivation

In this tutorial, I'll be demonstrating how to create a template for a fullstack web app using [Axum](https://github.com/tokio-rs/axum) as the backend in Rust, and React with [Vite](https://vitejs.dev/) for the frontend.

We'll also be taking advantage of the Typescript type generating tool we built in a previous tutorial to generate TS types from Rust code and share types between our frontend and backend.  This is a common benefit of working with a Node.js driven backend, but more difficult to achieve when building in another language (like Rust!)


# Project Setup

If you're following along after completing the previous tutorial, then I suggest you create your app in a folder right next to your type generation CLI so that you can easily invoke it without dealing with painful file paths.  

Alternatively if you published it, you can install the utility with `cargo install`, or simply install the demo one I created for the tutorial with `cargo install typester`.

_(You can even follow this tutorial simply for the template setup and not use the type sharing utility at all if you choose)_

Open your terminal where you'd like to create the project and run this command:

```
cargo new fullstack-app
cd fullstack-app
```

Open up `/fullstack-app` in your IDE and add the following dependencies to your `Cargo.toml` file:

```toml
[package]
name = "fullstack-app"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.5.16"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.68"
tokio = { version = "1.0", features = ["full"] }
tower-http = { version = "0.3.4", features = ["cors"] }
```

In addition to `axum` for the web server, which is built on `tokio`, we also use `tower-http` to simplify CORS.

We will be using `serde` and `serde_json` for serialization of our Rust data to send to the frontend.  

# Rust Server

Begin by opening up `main.rs` and replacing it with this simple template for a web server with returns _"Hello, world!"_ at the root route: 


`fullstack-app/src/main.rs`
```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
```

If you are familiar with web servers in other languages this should all look pretty familiar.  The `root()` function is the route handler for `/` which is set here:

```rust
let app = Router::new()
    .route("/", get(root))
    .layer(cors);
```

We'll be using a very permissive set of rules for CORS to avoid any issues testing locally, though you'll likely want to adjust this before you publish the app live.

Let's test it out!  Start your server with:

```
cargo run
```

You should be able to visit [http://localhost:3000]() and see the _**"Hello, World!"**_ response from your root route.

Once you have that working we'll add a route that returns some serialized Rust data. 

First we'll create a second file in `src` called `types.rs` similar to how we did in the Typester project.

Populate it with the following type:

`fullstack-app/src/types.rs`
```rust
use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub favourite_food: Option<String>
}
```

We're going to keep it fairly simple for this demonstration, just a basic struct. You can add more complex types later if you like.

Now let's update our server code to include a GET route that returns a serialized vector of Person structs. 

When this is deserialized in Typescript the front end should expect to receive an array of Person objects.

`fullstack-app/src/main.rs`

```rust
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// NEW
mod types;
use types::Person;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))

        // NEW
        .route("/people", get(get_people))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

// NEW
async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
    ];

    (StatusCode::OK, Json(people))
}

```

There are three blocks in the above I've annotated as `// NEW` to indicate the changes we have made to the origin example of `main.rs`.

We've imported the types from the `types.rs` file, created a new route function called `get_people` which returns a vector of example people as JSON (the `Json` struct takes care of the serialization for us using `serde` in the background).

# React Client

Now let's create the front end client that will access the server we just created.

From your `/fullstack-app` directory run the following command (make sure you have [NodeJs](https://nodejs.org/en/) installed first):

```
npm create vite@latest
```

For project name choose `client` and for the template choose `React` with `Typescript`.

Open up `App.tsx` and replace the sample code with the following:

`fullstack-app/client/src/App.tsx`

```tsx
import { useEffect, useState } from "react";
import { Person } from "./types";

function App() {
  const [people, setPeople] = useState<Person[]>([]);

  useEffect(() => {
    fetch("http://localhost:3000/people")
      .then((res) => res.json())
      .then((people: Person[]) => setPeople(people));
  }, []);

  return (
    <div>
      {people.map((person) => (
        <p>
          {person.name} is {person.age} years old
        </p>
      ))}
    </div>
  );
}

export default App;
```

You'll have an error.  Where is the `Person` type coming from?  The `types` file it's trying to import is not defined.

We'll use our typegen utility to create it!

Now the way that you use it is really going to depend on a few factors, such as whether you did the previous tutorial, and whether you published it.  If you didn't do it at all, the third option will be to simply use the version I published with the post:

### Option 1: Run your local copy

Let's say you have your typegen utility in a folder adjacent to this app like so:

```
/YOUR_TYPEGEN_PROGRAM
/fullstack-app
```

Then you will invoke it with the following command from the `/YOUR_TYPEGEN_PROGRAM` directory (or whatever you named yours):

```
cargo run -- --input=../fullstack-app/src/types.rs --output=../fullstack-app/client/src/types.d.ts
```

### Option 2: Run your published copy

If you published your copy to `crates.io` then you can install it on your machine with:

```
cargo install YOUR_TYPEGEN_PROGRAM
```

And then from within the `/fullstack-app` directory:

```
YOUR_TYPEGEN_PROGRAM --input=./src/types.rs --output=./client/src/types.d.ts
```

### Option 3: Run my Published Copy

If you didn't follow the previous tutorial you can [simply use mine](https://crates.io/crates/typester).

Just repeat `Option 2` with `typester` in place of `YOUR_TYPEGEN_PROGRAM`.

---

If all goes well you'll have a generated file with the following filename and content:

`client/src/types.d.ts`
```ts
type HashSet<T extends number | string> = Record<T, undefined>;
type HashMap<T extends number | string, U> = Record<T, U>;
type Vec<T> = Array<T>;
type Option<T> = T | undefined;
type Result<T, U> = T | U;

export interface Person {
  name: string;
  age: number;
  favourite_food: Option<string>;
}
```

With this type file in place you should no longer see any error in your `App.tsx` file and `person` should be correctly typed and identifiable as a `Person` by your IDE for type checking and intellisense.  

Here's a look at how VS Code will interpret it:

![Type Hint Example](https://res.cloudinary.com/dqse2txyi/image/upload/v1666050136/axum_server/ts_example_chvbjf.png)

# Add Styles

Just before we test out the finished version live, let's just it a quick pass with some CSS and turn these people into some basic employee cards.

We'll be straight up pulling it directly from [this simple example here](https://www.w3schools.com/howto/howto_css_cards.asp):

Create a file called `App.css` directly beside `App.tsx` (or override the one that is included with the project by default) and add this CSS:

`client/src/App.css`
```css
.app {
  display: flex;
  flex-direction: row;
  column-gap: 16px;
}

.card {
  box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
  transition: 0.3s;
  max-width: 300px;
}

.card:hover {
  box-shadow: 0 8px 16px 0 rgba(0, 0, 0, 0.2);
}

.container {
  padding: 2px 16px;
}

img {
  width: 300px;
}
```

Next, update your `App.tsx` to include the following.  You can feel free to change the avatar URLs if you choose.  Make sure you don't miss the `import "App.css";` in there!

```client/src/App.tsx`
```tsx
import { useEffect, useState } from "react";
import { Person } from "./types";
import "./App.css";

const AVATAR_1 =
  "https://res.cloudinary.com/dqse2txyi/image/upload/v1666049372/axum_server/img_avatar_lf92vl.png";
  
const AVATAR_2 =
  "https://res.cloudinary.com/dqse2txyi/image/upload/v1666049372/axum_server/img_avatar2_erqray.png";

function App() {
  const [people, setPeople] = useState<Person[]>([]);

  useEffect(() => {
    fetch("http://localhost:3000/people")
      .then((res) => res.json())
      .then((people: Person[]) => setPeople(people));
  }, []);

  return (
    <div className="app">
      {people.map((person, index) => (
        <div key={index} className="card">
          <img src={index % 2 == 0 ? AVATAR_1 : AVATAR_2} alt="Avatar" />
          <div className="container">
            <h4>
              <b>{person.name}</b>
            </h4>
            <p>Age: {person.age}</p>
            <p>Favourite Food: {person.favourite_food ?? "Unknown"}</p>
          </div>
        </div>
      ))}
    </div>
  );
}

export default App;

```

Now, start up your Rust web server:

_(Make sure you are in the `/fullstack-app` directory!)_

```
cargo run
```

You should see this output:

```
listening on 127.0.0.1:3000
```

Next start your front end Vite dev server.

_(Make sure you are in the `/fullstack-app/client` directory!)_

```
npm run dev
```

If you see any errors on these commands, double check again that you are running them from the directories listed above.

With both running visit the default Vite server URL/port at:

[http://localhost:5173/]()

Enjoy your beautiful new app!

![People Cards](https://res.cloudinary.com/dqse2txyi/image/upload/v1666050412/axum_server/people_cards_kjur37.png)


# Conclusion

I hope you've learned something new about building a web app using the best of both worlds (Rust and Typescript).  I would encourage you to keep developing on this idea and see what additional features you can build!

That said if you are looking for something more production ready, I would highly encourage you to check out Wulf's [Create Rust App](https://github.com/Wulf/create-rust-app), a fantastic template built in the same vein of this project, but much more feature complete.



