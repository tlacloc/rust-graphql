# Graphql API

based on this [blog](https://dev.to/open-graphql/building-powerful-graphql-servers-with-rust-3gla)

# Steps to deploy to Heroku

Build a Dockerfile (one is already created in the repository), you can build the image using the following command

```bash
docker build -t rust-graphql:latest .
```

it takes a while to build, but after that you should be able to run it locally with the following command

```bash
docker run -d --name rust-graphql -e "PORT=8765" -e "DEBUG=0" -p 8007:8765 rust-graphql:latest
```
