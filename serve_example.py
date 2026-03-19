# For Wasm, CORS policy bla bla

import http.server, socketserver;

class handler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path in ['/', '/example.html']:
            self.path='example.html'
        return super().do_GET()

with socketserver.TCPServer(('', 0), handler) as httpd: 
    print(f'\033[1mServing at http://localhost:{httpd.server_address[1]}/example.html\033[0m'); 
    httpd.serve_forever()
