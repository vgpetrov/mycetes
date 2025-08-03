# mycetes
Run docker compose in separate project `docker-compose -f docker-compose.yml -p mycetes up`

# How to check the project name of a container
`docker ps --format 'table {{.Names}}\t{{.Labels}}'`
Look for labels like:
com.docker.compose.project=projectA