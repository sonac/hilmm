# HILMM

How I Lost My Money

## How to run?

`docker build . -t hilmm `

provide necessary env variables (e.g. in a .env file)

```
MONGO_USER=foo
MONGO_PASSWORD=password
MONGOURI=mongodb://foo:password@localhost:27017/hilmm?retryWrites=true&w=majority
ROCKET_SECRET_KEY=111111111111111111111111111111111111
```

`docker run -p 8000:8000 --env-file .env hilmm`
