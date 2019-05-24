# Tutorial CENNZnet docker
Build and run a cennznet docker image (for tutorial only)
```bash
curl -sL https://github.com/cennznet/spin2win/releases/download/0.0.1a/cennznet-docker.tar.gz | tar xz

docker build -t cennznet .

docker run \
  --name cennznet \
  -p 9944:9944 \
  -p 30333:30333 \
  cennznet --dev
```
