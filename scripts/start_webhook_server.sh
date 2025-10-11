#!/bin/bash

# Helper script to start a local webhook server for testing
# This script starts a simple Python HTTP server and optionally exposes it via tunnelto/ngrok
#
# Usage:
#   ./start_webhook_server.sh [PORT]
#
# Optional Environment Variables:
#   TUNNELTO_SUBDOMAIN - Use a custom tunnelto subdomain (requires subscription)
#                        Example: export TUNNELTO_SUBDOMAIN="my-webhook"
#                        Will create: https://my-webhook.tunn.dev
#
# Without TUNNELTO_SUBDOMAIN, a random subdomain is used (no subscription needed)

set -e

PORT=${1:-8080}

echo "🌐 Starting local webhook server on port $PORT..."
echo ""

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Error: python3 is required but not installed."
    exit 1
fi

# Start a simple HTTP server that responds to all requests with 200 OK
echo "Starting Python HTTP server..."
cat > /tmp/webhook_server.py << 'EOF'
from http.server import HTTPServer, BaseHTTPRequestHandler
import sys
import json

class WebhookHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        response = json.dumps({'status': 'ok', 'message': 'Webhook endpoint is ready'})
        response_bytes = response.encode('utf-8')
        
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.send_header('Content-Length', str(len(response_bytes)))
        self.send_header('Connection', 'close')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(response_bytes)
        print(f"✅ Received GET request: {self.path}")
    
    def do_POST(self):
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length) if content_length > 0 else b''
        
        response = json.dumps({'status': 'ok', 'message': 'Webhook received'})
        response_bytes = response.encode('utf-8')
        
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.send_header('Content-Length', str(len(response_bytes)))
        self.send_header('Connection', 'close')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(response_bytes)
        print(f"✅ Received POST request: {self.path}")
        
        # Log Circle webhook signature headers if present
        circle_signature = self.headers.get('X-Circle-Signature')
        circle_key_id = self.headers.get('X-Circle-Key-Id')
        if circle_signature or circle_key_id:
            print(f"   🔐 Circle Webhook Headers:")
            if circle_key_id:
                print(f"      X-Circle-Key-Id: {circle_key_id}")
            if circle_signature:
                print(f"      X-Circle-Signature: {circle_signature[:50]}...")
        
        if body:
            print(f"   Body: {body.decode('utf-8', errors='ignore')[:200]}")
    
    def do_HEAD(self):
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.send_header('Content-Length', '0')
        self.send_header('Connection', 'close')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        print(f"✅ Received HEAD request: {self.path}")
    
    def log_message(self, format, *args):
        pass  # Suppress default logging

port = int(sys.argv[1]) if len(sys.argv) > 1 else 8080
server = HTTPServer(('localhost', port), WebhookHandler)
print(f"🎯 Webhook server listening on http://localhost:{port}")
print(f"   Ready to accept connections from tunnelto/ngrok")
print(f"   Press Ctrl+C to stop")
print("")
sys.stdout.flush()
server.serve_forever()
EOF

# Start the server in the background
python3 /tmp/webhook_server.py $PORT > /tmp/webhook_server.log 2>&1 &
SERVER_PID=$!

echo "⏳ Starting webhook server (PID: $SERVER_PID)..."

# Wait for server to start and verify it's running
sleep 2

# Verify the server is actually running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo "❌ Failed to start webhook server. Server process died."
    echo "Check /tmp/webhook_server.log for errors:"
    cat /tmp/webhook_server.log
    exit 1
fi

# Test that the server is responding - retry up to 5 times
if command -v curl &> /dev/null; then
    echo "🔍 Testing local server connectivity..."
    
    MAX_RETRIES=5
    RETRY_COUNT=0
    SERVER_READY=false
    
    while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
        if curl -s -f -m 5 http://localhost:$PORT/ > /dev/null 2>&1; then
            SERVER_READY=true
            break
        fi
        RETRY_COUNT=$((RETRY_COUNT + 1))
        if [ $RETRY_COUNT -lt $MAX_RETRIES ]; then
            echo "   Attempt $RETRY_COUNT/$MAX_RETRIES failed, retrying..."
            sleep 1
        fi
    done
    
    if [ "$SERVER_READY" = true ]; then
        echo "✅ Local server is responding on http://localhost:$PORT"
    else
        echo "❌ Local server is not responding after $MAX_RETRIES attempts"
        echo "Server process status: $(ps -p $SERVER_PID -o state= 2>/dev/null || echo 'not running')"
        echo "Check /tmp/webhook_server.log:"
        cat /tmp/webhook_server.log
        echo ""
        echo "Trying to see what's on port $PORT:"
        lsof -i :$PORT 2>/dev/null || netstat -tlnp 2>/dev/null | grep ":$PORT" || echo "Port check tools not available"
        exit 1
    fi
    echo ""
else
    echo "⚠️  curl not found, skipping server health check"
    echo ""
fi

echo "✅ Webhook server ready (PID: $SERVER_PID)"
echo ""

TUNNEL_PID=""

# Check if tunnelto is available (preferred)
if command -v tunnelto &> /dev/null; then
    echo "🚀 tunnelto detected! Starting tunnel..."
    echo ""
    
    # Check if user has a custom subdomain (requires tunnelto subscription)
    CUSTOM_SUBDOMAIN="${TUNNELTO_SUBDOMAIN:-}"
    
    if [ -n "$CUSTOM_SUBDOMAIN" ]; then
        echo "📌 Using custom subdomain: $CUSTOM_SUBDOMAIN"
        # Start tunnelto with custom subdomain (requires subscription)
        # Use 127.0.0.1 instead of localhost to force IPv4
        tunnelto --host 127.0.0.1 --port $PORT --subdomain "$CUSTOM_SUBDOMAIN" > /tmp/tunnelto.log 2>&1 &
        TUNNEL_PID=$!
    else
        echo "🎲 Using random subdomain (no subscription needed)"
        # Start tunnelto without subdomain (uses random, no reservation needed)
        # Use 127.0.0.1 instead of localhost to force IPv4
        tunnelto --host 127.0.0.1 --port $PORT > /tmp/tunnelto.log 2>&1 &
        TUNNEL_PID=$!
    fi
    
    # Wait for tunnelto to start and establish connection
    echo "⏳ Waiting for tunnel to establish..."
    sleep 5
    
    # Check for errors first
    if grep -q "Error:" /tmp/tunnelto.log; then
        echo "❌ tunnelto error:"
        cat /tmp/tunnelto.log
        echo ""
        
        # Check if it's a subscription limit error
        if grep -q "maximum number of reserved subdomains" /tmp/tunnelto.log; then
            echo "💡 You have a tunnelto subscription with custom domains!"
            echo "   Set your custom subdomain:"
            echo "   export TUNNELTO_SUBDOMAIN=\"your-custom-subdomain\""
            echo "   Then run this script again."
            echo ""
            echo "   Or use webhook.site (no account needed):"
            echo "   1. Visit https://webhook.site"
            echo "   2. Copy your unique URL"
            echo "   3. export CIRCLE_TEST_WEBHOOK_URL=\"https://webhook.site/your-id\""
        else
            echo "💡 Try using webhook.site instead:"
            echo "   1. Visit https://webhook.site"
            echo "   2. Copy your unique URL"
            echo "   3. export CIRCLE_TEST_WEBHOOK_URL=\"https://webhook.site/your-id\""
        fi
        kill $TUNNEL_PID 2>/dev/null
        exit 1
    fi
    
    # Extract the URL from the log (handles both old format and new table format)
    # Match URLs like https://xyz.tunn.dev but not https://dashboard.tunnelto.dev
    PUBLIC_URL=$(grep -oE 'https://[a-z0-9-]+\.tunn\.dev' /tmp/tunnelto.log | head -1)
    
    # If that didn't work, try the alternative pattern
    if [ -z "$PUBLIC_URL" ]; then
        PUBLIC_URL=$(grep -oE 'https://[a-z0-9-]+\.tunnelto\.dev' /tmp/tunnelto.log | head -1)
    fi
    
    if [ -n "$PUBLIC_URL" ] && [[ ! "$PUBLIC_URL" =~ dashboard ]]; then
        echo "✅ tunnelto tunnel established!"
        echo ""
        
        # Test the tunnel endpoint
        if command -v curl &> /dev/null; then
            echo "🔍 Testing tunnel endpoint..."
            if curl -s -f --max-time 10 "$PUBLIC_URL" > /dev/null 2>&1; then
                echo "✅ Tunnel is responding correctly"
            else
                echo "⚠️  Warning: Tunnel may not be responding yet. Wait a few seconds and try again."
            fi
            echo ""
        fi
        
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "🌍 Public webhook URL: $PUBLIC_URL"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "💡 To use this URL in your tests, run:"
        echo "   export CIRCLE_TEST_WEBHOOK_URL=\"$PUBLIC_URL\""
        echo ""
        echo "⏰ Wait 5-10 seconds before running tests to ensure tunnel is fully ready"
        echo ""
    else
        echo "⚠️  Could not extract tunnelto URL. Check /tmp/tunnelto.log"
        echo ""
        echo "Log contents:"
        cat /tmp/tunnelto.log
        echo ""
        echo "💡 Try using webhook.site instead:"
        echo "   1. Visit https://webhook.site"
        echo "   2. Copy your unique URL"
        echo "   3. export CIRCLE_TEST_WEBHOOK_URL=\"https://webhook.site/your-id\""
        kill $TUNNEL_PID 2>/dev/null
        exit 1
    fi
# Check if ngrok is available (fallback)
elif command -v ngrok &> /dev/null; then
    echo "🚀 ngrok detected! Starting tunnel..."
    echo ""
    
    # Start ngrok
    ngrok http $PORT --log=stdout > /tmp/ngrok.log 2>&1 &
    TUNNEL_PID=$!
    
    # Wait for ngrok to start and get the URL
    sleep 3
    
    # Extract the public URL
    PUBLIC_URL=$(curl -s http://localhost:4040/api/tunnels | grep -o 'https://[^"]*\.ngrok-free\.app' | head -1)
    
    if [ -z "$PUBLIC_URL" ]; then
        PUBLIC_URL=$(curl -s http://localhost:4040/api/tunnels | grep -o 'https://[^"]*\.ngrok\.io' | head -1)
    fi
    
    if [ -n "$PUBLIC_URL" ]; then
        echo "✅ ngrok tunnel established!"
        echo ""
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "🌍 Public webhook URL: $PUBLIC_URL"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "💡 To use this URL in your tests, run:"
        echo "   export CIRCLE_TEST_WEBHOOK_URL=\"$PUBLIC_URL\""
        echo ""
        echo "📊 ngrok dashboard: http://localhost:4040"
        echo ""
    else
        echo "⚠️  Could not get ngrok URL. Check http://localhost:4040"
        echo ""
    fi
else
    echo "ℹ️  No tunnel tool found. Local server is only accessible at http://localhost:$PORT"
    echo ""
    echo "💡 To expose this server publicly, install a tunnel tool:"
    echo ""
    echo "   Option 1: tunnelto (Open Source, Recommended)"
    echo "      curl -sL https://tunnelto.dev/install.sh | sh"
    echo "      Then run: tunnelto --port $PORT"
    echo "      More info: https://tunnelto.dev/"
    echo ""
    echo "   Option 2: ngrok"
    echo "      Download from: https://ngrok.com/download"
    echo "      Then run: ngrok http $PORT"
    echo ""
    echo "   Option 3: Use a webhook testing service"
    echo "      https://webhook.site (easiest, no install needed)"
    echo ""
fi

echo "Press Ctrl+C to stop the server"
echo ""

# Cleanup function
cleanup() {
    echo ""
    echo "🛑 Stopping webhook server..."
    kill $SERVER_PID 2>/dev/null || true
    if [ -n "$TUNNEL_PID" ]; then
        kill $TUNNEL_PID 2>/dev/null || true
        echo "🛑 Stopping tunnel..."
    fi
    rm -f /tmp/webhook_server.py /tmp/webhook_server.log /tmp/tunnelto.log /tmp/ngrok.log
    echo "✅ Cleanup complete"
    exit 0
}

# Register cleanup function
trap cleanup SIGINT SIGTERM

# Wait for the server process
wait $SERVER_PID

