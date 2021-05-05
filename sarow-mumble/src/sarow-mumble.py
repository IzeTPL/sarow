from urllib.parse import urlparse
from http.server import BaseHTTPRequestHandler, HTTPServer
from pymumble_py3 import Mumble

mumble_client = Mumble("iktelpan.ishfid.pl", "Sarow")


class HttpRequestHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        print(self.request)
        print(self.client_address)
        url = urlparse(self.path)
        #channel = mumble_client.my_channel()
        #channel.send_text_message(url.query.split("=")[1])
        print(url.query.split("=")[1])
        self.send_response(200)
        self.end_headers()


def main():
    server = HTTPServer(("0.0.0.0", 8081), HttpRequestHandler)
    #mumble_client.start()
    #mumble_client.connect()
    #mumble_client.is_ready()

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("Shutting down")
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
