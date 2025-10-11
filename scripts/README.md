# Helper Scripts

This directory contains helper scripts for development and testing.

## start_webhook_server.sh

A helper script that starts a local webhook server for testing notification subscriptions.

### Features

- Starts a simple HTTP server that responds with 200 OK to all requests
- Automatically detects and uses tunnelto or ngrok (if available) to expose the server publicly
- Provides the public URL to use in tests
- Prefers tunnelto (open source) over ngrok

### Usage

```bash
# Start on default port 8080
./scripts/start_webhook_server.sh

# Start on custom port
./scripts/start_webhook_server.sh 3000
```

### With tunnelto (Recommended)

If [tunnelto](https://tunnelto.dev/) is installed, the script will automatically:
1. Start the local webhook server
2. Create a tunnelto tunnel to expose it publicly
3. Display the public URL to use in your tests

```bash
./scripts/start_webhook_server.sh

# Output will show:
# 🌍 Public webhook URL: https://circle-test-123456.tunnelto.dev
#
# 💡 To use this URL in your tests, run:
#    export CIRCLE_TEST_WEBHOOK_URL="https://circle-test-123456.tunnelto.dev"
```

Then in another terminal:

```bash
export CIRCLE_TEST_WEBHOOK_URL="https://circle-test-123456.tunnelto.dev"
cargo test test_notification_subscriptions_crud
```

#### Custom Subdomain (Optional)

You can set a custom subdomain for your tunnelto tunnel by setting the `TUNNELTO_SUBDOMAIN` environment variable:

```bash
# Set a custom subdomain
export TUNNELTO_SUBDOMAIN="my-circle-webhook"
./scripts/start_webhook_server.sh

# This will create: https://my-circle-webhook.tunnelto.dev
```

Or set it inline:

```bash
TUNNELTO_SUBDOMAIN="my-circle-webhook" ./scripts/start_webhook_server.sh
```

**Benefits of custom subdomains:**
- Easier to remember and identify your webhook URL
- Consistent URL across multiple test runs
- Makes debugging webhook calls simpler

**Note:** Custom subdomains require a tunnelto account (free to create at https://tunnelto.dev)

### With ngrok

The script also supports ngrok as a fallback. If tunnelto is not installed but ngrok is, it will automatically use ngrok.

### Without a tunnel tool

If neither tunnelto nor ngrok is installed, you can:

1. Install tunnelto (recommended, open source):
   ```bash
   curl -sL https://tunnelto.dev/install.sh | sh
   ```

2. Install ngrok: https://ngrok.com/download

3. Or use https://webhook.site as an alternative (easiest, no install needed)

### Requirements

- Python 3 (for the local HTTP server) - **Required**
- tunnelto (optional, but recommended for testing with Circle's API) - Install from [tunnelto.dev](https://tunnelto.dev/)
- ngrok (optional, fallback option) - Download from [ngrok.com](https://ngrok.com/download)

### Stopping the Server

Press `Ctrl+C` to stop both the local server and any active tunnel (tunnelto or ngrok).

