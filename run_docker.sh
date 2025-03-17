docker stop algorithmics
docker rm algorithmics
docker build . -t algorithmics --network host
docker run --name algorithmics -d -i -t algorithmics /bin/sh

# Remove dangling images if any exist
if [ "$(docker images -f dangling=true -q)" ]; then
  docker rmi $(docker images -f dangling=true -q)
fi

docker exec -it algorithmics /bin/sh