```bash
# Install dependencies
npm install

# Generate a client so it can talk to the server
npm run generate-client

# Run locally with hot reload
npm run dev
```

## Add credentials

In the `.env` file, add

```
VITE_GOOGLE_MAPS_API_KEY=<redacted>
VITE_API_URL=<provided by the server upload>
```

## Updating packages

```bash
ncu
ncu -u
```
