#!/usr/bin/env python3

import http
import os
from http.server import BaseHTTPRequestHandler
import socketserver

PORT = int(os.getenv('HTTP_PORT') or 8081)


class SimpleHandler(BaseHTTPRequestHandler):
	def do_GET(self):
		self.handle_request()

	def do_POST(self):
		self.handle_request()

	def handle_request(self):
		print("================================")
		print("Request line:", self.requestline)
		print("Headers:\n")
		print(self.headers)

		length = int(self.headers['Content-Length'] or 0)
		body = self.rfile.read(length)
		print(str(body, encoding='utf-8'))
		print("================================")

		self.send_response(http.HTTPStatus.OK)
		self.send_header("Content-Type", "plain/text")
		self.end_headers()
		self.wfile.write(b"OK")


if __name__ == '__main__':
	with socketserver.TCPServer(("0.0.0.0", PORT), SimpleHandler) as httpd:
		print("Server has started at", httpd.server_address[0] + ":" + str(httpd.server_address[1]))
		httpd.serve_forever()
