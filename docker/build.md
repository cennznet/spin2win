# Tutorial CENNZnet docker
Build and run a cennznet docker image (for tutorial only)
```bash
docker build -t cennznet .
docker run \
  --name cennznet \
  -p 9944:9944 \
  -p 30333:30333 \
  cennznet --dev
```
