# Mycetes docker compose
Run docker compose in separate project

```aiignore
docker-compose -f docker-compose.yml -p mycetes up
docker-compose -f docker-compose.yml down --rmi all
docker-compose -f docker-compose.yml up --build
```

# Mycetes Docker build only
`docker build -t mycetes -f misc/docker/Dockerfile .`

# How to check the project name of a container
`docker ps --format 'table {{.Names}}\t{{.Labels}}'`
Look for labels like:
com.docker.compose.project=projectA



# Enum based polymorphism (closed polymorphism)
```aiignore
trait Repository {
    fn list_places(&self) -> Vec<Place>;
}

struct MemRepository {}
struct DbRepository {}

impl Repository for MemRepository {
    fn list_places(&self) -> Vec<Place> {
        todo!()
    }
}

impl Repository for DbRepository {
    fn list_places(&self) -> Vec<Place> {
        todo!()
    }
}

enum Repo {
    Mem(MemRepository),
    Db(DbRepository)
}

impl Repo {

    fn list_places(&self) -> Vec<Place> {
        match self {
            Repo::Mem(r) => { r.list_places()}
            Repo::Db(r) => { r.list_places()}
        }
    }
}
```