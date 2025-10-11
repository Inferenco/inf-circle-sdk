# Testing Guide

This guide explains how to run the integration tests for the Circle SDK.

## Overview

The SDK includes comprehensive integration tests that validate functionality against Circle's actual API. The tests are designed to be practical and handle real-world scenarios.

## Quick Start

### 1. Set Up Environment Variables

Create a `.env` file in the project root with your Circle API credentials:

```bash
CIRCLE_BASE_URL="https://api.circle.com"
CIRCLE_API_KEY="your_api_key_here"
CIRCLE_ENTITY_SECRET="your_entity_secret_hex"
CIRCLE_PUBLIC_KEY="-----BEGIN PUBLIC KEY-----
your_rsa_public_key_here
-----END PUBLIC KEY-----"
CIRCLE_WALLET_SET_ID="your_wallet_set_id"
CIRCLE_TEMPLATE_ID="your_contract_template_id"
```

### 2. Run Tests

```bash
# Run all tests
cargo test

# Run with output visible
cargo test -- --nocapture

# Run specific test
cargo test test_notification_subscriptions_crud -- --nocapture
```

## Webhook Testing

The notification subscription tests require special setup because Circle validates webhook endpoints by making HTTP requests to them.

### Problem

When you create a notification subscription, Circle's API:
1. Validates the endpoint URL
2. Makes an HTTP request to the endpoint
3. Expects a 2xx response code
4. Rejects the subscription if the endpoint is not reachable

This means you can't use `localhost` URLs or fake URLs like `https://example.com`.

### Solution: Three Options

#### Option 1: tunnelto (Best for Development - Open Source)

[tunnelto](https://tunnelto.dev/) is an open-source tool that exposes your local server to the internet.

1. Install tunnelto:
```bash
curl -sL https://tunnelto.dev/install.sh | sh
```

2. Start the helper script:
```bash
./scripts/start_webhook_server.sh
```

The script will:
- Start a local Python HTTP server on port 8080
- Launch tunnelto to expose it publicly
- Display the public URL to use

3. Copy the public URL and export it:
```bash
export CIRCLE_TEST_WEBHOOK_URL="https://circle-test-123456.tunnelto.dev"
```

4. Run your tests in another terminal:
```bash
cargo test test_notification_subscriptions_crud -- --nocapture
```

**Pros:**
- Open source software
- Full control over the webhook server
- Can see all incoming requests in the server logs
- Great for debugging
- Private tunnel to your local machine
- Free to use

**Cons:**
- Requires installation (one-time)
- URLs change each time you restart (unless you use a paid custom subdomain)

#### Option 2: webhook.site (Easiest - No Installation Required)

1. Visit https://webhook.site
2. Copy your unique webhook URL (e.g., `https://webhook.site/#!/abc-123-def`)
3. Use the base URL (without the `#!/` part) as your webhook endpoint:

```bash
export CIRCLE_TEST_WEBHOOK_URL="https://webhook.site/abc-123-def"
cargo test test_notification_subscriptions_crud -- --nocapture
```

**Pros:**
- No setup or installation required
- Works immediately
- You can see webhook payloads in your browser

**Cons:**
- URL is public (anyone with the URL can see payloads)
- URL expires after a period of inactivity
- Less control over the server

#### Option 3: ngrok (Alternative Tunnel Tool)

1. Install ngrok: https://ngrok.com/download

2. Start the helper script (it will detect ngrok automatically):
```bash
./scripts/start_webhook_server.sh
```

3. Copy the public URL and export it:
```bash
export CIRCLE_TEST_WEBHOOK_URL="https://abc123.ngrok-free.app"
```

4. Run your tests:
```bash
cargo test test_notification_subscriptions_crud -- --nocapture
```

**Pros:**
- Full control over the webhook server
- Web dashboard at http://localhost:4040
- More mature product

**Cons:**
- Free tier has request limits
- URLs change each time (unless you have a paid account)
- Not open source

#### Option 4: Skip the Test

If you don't want to set up webhooks, simply don't set `CIRCLE_TEST_WEBHOOK_URL`. The test will automatically skip with a helpful message:

```
⚠️  CIRCLE_TEST_WEBHOOK_URL not set. Skipping notification subscription test.
   To run this test, set a publicly accessible webhook URL:
   export CIRCLE_TEST_WEBHOOK_URL="https://webhook.site/your-unique-id"
   You can get a free webhook URL from https://webhook.site
```

## Test Structure

### Wallet Tests (`wallets_integration_test.rs`)

Tests for wallet creation, listing, transfers, and transactions:
- `test_create_and_list_wallets`
- `test_create_transfer_transaction`
- `test_estimate_transfer_fee`
- etc.

### Contract Tests (`contracts_integration_test.rs`)

Tests for smart contract deployment and management:
- `test_ping`
- `test_list_contracts`
- `test_deploy_contract_from_template`
- `test_estimate_template_deployment_fee`
- `test_notification_subscriptions_crud` (requires webhook URL)

## Common Issues

### "Failed to verify endpoint" Error

```
Failed to create notification subscription: Api { status: 400, message: "Failed to verify endpoint https://example.com/webhook/... because non-2xx status code (4xx)" }
```

**Solution:** You need to provide a publicly accessible webhook endpoint. See the Webhook Testing section above.

### Rate Limiting

Circle's API has rate limits. The tests include automatic retry logic with exponential backoff for 429 errors, but if you hit rate limits frequently:

- Wait a few minutes between test runs
- Run specific tests instead of the entire suite
- Contact Circle support to increase your rate limits

### Missing Environment Variables

```
Failed to create CircleView: EnvVar("Missing environment variable: CIRCLE_API_KEY")
```

**Solution:** Make sure your `.env` file is in the project root and contains all required variables.

## Test Best Practices

1. **Reuse Wallets:** The tests automatically reuse wallets based on blockchain and ref_id to avoid creating too many wallets. Fund your test wallets once and they'll be reused across test runs.

2. **Fund Test Wallets:** Some tests require funded wallets (e.g., transfer tests). Use testnet faucets:
   - Ethereum Sepolia: https://sepoliafaucet.com/
   - Polygon Amoy: https://faucet.polygon.technology/
   - Avalanche Fuji: https://faucet.avax.network/

3. **Clean Up:** Webhook subscriptions created during tests are automatically deleted at the end of the test.

4. **Run Tests Individually:** When developing or debugging, run specific tests to avoid hitting rate limits:
   ```bash
   cargo test test_notification_subscriptions_crud -- --nocapture
   ```

## Helper Scripts

### `scripts/start_webhook_server.sh`

Starts a local webhook server and optionally exposes it via ngrok.

```bash
# Start with default port (8080)
./scripts/start_webhook_server.sh

# Start with custom port
./scripts/start_webhook_server.sh 3000

# Stop with Ctrl+C
```

See `scripts/README.md` for more details.

## Troubleshooting

### "5xx" Error When Creating Subscription

If you get an error like:
```
Failed to verify endpoint https://circle-test-123.tunn.dev because non-2xx status code (5xx)
```

This means Circle reached your tunnel but got a server error. **Solutions:**

1. **Wait longer before running tests** - The tunnel needs time to stabilize:
   ```bash
   # After starting the webhook server and exporting the URL
   sleep 10
   # Now run tests
   cargo test test_notification_subscriptions_crud -- --nocapture
   ```

2. **Test the tunnel manually first**:
   ```bash
   curl https://your-tunnel-url.tunnelto.dev
   # Should return: {"status":"ok","message":"Webhook endpoint is ready"}
   ```
   
   If you get an error, wait a few seconds and try again.

3. **Check the local server logs**:
   ```bash
   cat /tmp/webhook_server.log
   ```
   
   Look for Python errors or issues binding to the port.

4. **Verify tunnelto is running**:
   ```bash
   ps aux | grep tunnelto
   cat /tmp/tunnelto.log
   ```

5. **Restart everything**:
   ```bash
   # Stop the current server (Ctrl+C in the first terminal)
   # Start fresh
   ./scripts/start_webhook_server.sh
   # Wait for "Tunnel is responding correctly" message
   # Wait an additional 10 seconds
   # Then run tests
   ```

### tunnelto Connection Issues

If tunnelto fails to connect:
1. Check your internet connection
2. Verify tunnelto is installed: `tunnelto --version`
3. Check `/tmp/tunnelto.log` for error messages
4. Try restarting: Press Ctrl+C and run the script again
5. If subdomain is taken, the script will auto-generate a new one on restart

### ngrok Connection Issues

If ngrok fails to connect:
1. Check your internet connection
2. Verify ngrok is installed: `ngrok version`
3. Ensure port 4040 is available (ngrok dashboard)
4. Try restarting ngrok

### Webhook.site Not Working

If webhook.site isn't working:
1. Make sure you're using the base URL (without `#!/`)
2. Try refreshing the webhook.site page
3. Get a new URL if the old one expired
4. Check your internet connection

### Python Server Issues

If the Python server fails to start:
1. Verify Python 3 is installed: `python3 --version`
2. Check if the port is already in use: `lsof -i :8080`
3. Try a different port: `./scripts/start_webhook_server.sh 3000`

### Tunnel URL Not Appearing

If the public URL isn't displayed:
1. Check the log files: `/tmp/tunnelto.log` or `/tmp/ngrok.log`
2. Make sure the tunnel tool is running (check process list)
3. Wait a few seconds more - sometimes tunnels take time to establish
4. Try the script again

## Additional Resources

- Circle Developer Docs: https://developers.circle.com/
- tunnelto: https://tunnelto.dev/ (recommended tunnel tool)
- webhook.site: https://webhook.site/ (quick webhook testing)
- ngrok Documentation: https://ngrok.com/docs (alternative tunnel tool)
- Project README: See `README.md` for SDK usage examples

## Contributing

When adding new tests:

1. Follow the existing test structure
2. Use the helper functions in `tests/common/mod.rs`
3. Handle rate limiting appropriately
4. Clean up resources after tests
5. Add documentation for any special setup requirements

For more details, see `CONTRIBUTING.md`.

