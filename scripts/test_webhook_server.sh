#!/bin/bash

# Diagnostic script to test if a webhook server can be started
# This helps troubleshoot issues before running the full script

set -e

PORT=${1:-8080}

echo "🔍 Webhook Server Diagnostic Test"
echo "=================================="
echo ""

# Check Python
echo "1. Checking Python..."
if command -v python3 &> /dev/null; then
    PYTHON_VERSION=$(python3 --version)
    echo "   ✅ Python found: $PYTHON_VERSION"
else
    echo "   ❌ Python3 not found"
    exit 1
fi
echo ""

# Check if port is available
echo "2. Checking if port $PORT is available..."
if lsof -i :$PORT > /dev/null 2>&1; then
    echo "   ❌ Port $PORT is already in use:"
    lsof -i :$PORT
    exit 1
elif netstat -tln 2>/dev/null | grep ":$PORT " > /dev/null; then
    echo "   ❌ Port $PORT is already in use:"
    netstat -tln | grep ":$PORT "
    exit 1
else
    echo "   ✅ Port $PORT is available"
fi
echo ""

# Create and start test server
echo "3. Starting test webhook server..."
cat > /tmp/test_webhook.py << 'EOF'
from http.server import HTTPServer, BaseHTTPRequestHandler
import sys
import json

class TestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps({'status': 'ok'}).encode())
    
    def do_HEAD(self):
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
    
    def log_message(self, format, *args):
        pass

port = int(sys.argv[1]) if len(sys.argv) > 1 else 8080
print(f"Starting server on 0.0.0.0:{port}")
sys.stdout.flush()
try:
    server = HTTPServer(('0.0.0.0', port), TestHandler)
    print(f"Server started successfully")
    sys.stdout.flush()
    server.serve_forever()
except Exception as e:
    print(f"Error: {e}")
    sys.exit(1)
EOF

python3 /tmp/test_webhook.py $PORT > /tmp/test_webhook.log 2>&1 &
TEST_PID=$!

echo "   Server PID: $TEST_PID"
sleep 2

# Check if process is running
if ! kill -0 $TEST_PID 2>/dev/null; then
    echo "   ❌ Server process died immediately"
    echo "   Log output:"
    cat /tmp/test_webhook.log
    rm -f /tmp/test_webhook.py /tmp/test_webhook.log
    exit 1
else
    echo "   ✅ Server process is running"
fi
echo ""

# Test connectivity
echo "4. Testing server connectivity..."
if command -v curl &> /dev/null; then
    sleep 1
    if curl -s -f -m 5 http://localhost:$PORT/ > /dev/null 2>&1; then
        echo "   ✅ Server is responding to HTTP requests"
        RESPONSE=$(curl -s http://localhost:$PORT/)
        echo "   Response: $RESPONSE"
    else
        echo "   ❌ Server is not responding to HTTP requests"
        echo "   Server log:"
        cat /tmp/test_webhook.log
        kill $TEST_PID 2>/dev/null
        rm -f /tmp/test_webhook.py /tmp/test_webhook.log
        exit 1
    fi
else
    echo "   ⚠️  curl not found, cannot test HTTP connectivity"
fi
echo ""

# Test with tunnelto if available
if command -v tunnelto &> /dev/null; then
    echo "5. Testing tunnelto connection..."
    SUBDOMAIN="test-$(date +%s)"
    
    tunnelto --host 127.0.0.1 --port $PORT --subdomain $SUBDOMAIN > /tmp/test_tunnel.log 2>&1 &
    TUNNEL_PID=$!
    
    echo "   Waiting for tunnel to establish..."
    sleep 5
    
    TUNNEL_URL=$(grep -o 'https://[^[:space:]]*\.tunnelto\.dev' /tmp/test_tunnel.log | head -1)
    
    if [ -n "$TUNNEL_URL" ]; then
        echo "   ✅ Tunnel established: $TUNNEL_URL"
        
        echo "   Testing tunnel endpoint..."
        sleep 2
        if curl -s -f -m 10 "$TUNNEL_URL" > /dev/null 2>&1; then
            echo "   ✅ Tunnel is working correctly!"
            RESPONSE=$(curl -s "$TUNNEL_URL")
            echo "   Response via tunnel: $RESPONSE"
        else
            echo "   ❌ Tunnel endpoint not responding"
            echo "   Tunnel log:"
            cat /tmp/test_tunnel.log
        fi
        
        kill $TUNNEL_PID 2>/dev/null
        rm -f /tmp/test_tunnel.log
    else
        echo "   ❌ Could not establish tunnel"
        cat /tmp/test_tunnel.log
        kill $TUNNEL_PID 2>/dev/null
        rm -f /tmp/test_tunnel.log
    fi
else
    echo "5. tunnelto not found, skipping tunnel test"
fi
echo ""

# Cleanup
echo "6. Cleaning up..."
kill $TEST_PID 2>/dev/null || true
rm -f /tmp/test_webhook.py /tmp/test_webhook.log
echo "   ✅ Cleanup complete"
echo ""

echo "=================================="
echo "✅ All diagnostics passed!"
echo ""
echo "Your system should be able to run the webhook server."
echo "Try running: ./scripts/start_webhook_server.sh"

