# byakuren.pw

## Post Format

Posts are written in markdown, but the first two lines of the file are special. 

The first line is the post "title", which will be displayed on the sidebar.

The second line is a timestamp showing when the post was written. To get this value, you can use `int(time.time())` in Python.

```md
Example Post Title
168685140

# Header

Post content
etc
```

## Docker

### Building

```shell
git clone https://github.com/Brod8362/byakuren.pw
docker build -t byakurenssg:latest .
```

### Running

```shell
docker run \
  -p 8000:8000 \
  -v /path/to/folder/with/posts:/app/posts \
  byakurenssg:latest
```