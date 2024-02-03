from bindings import exports

from urllib.request import urlopen


class Run(exports.Run):
    def run(self) -> None:
        print("Top of run")
        url_ptr = urlopen("https://www.rust-lang.org/logos/rust-logo-512x512.png")
        print("Read url ptr")
        print(url_ptr)
        content = url_ptr.read()
        print("Read content")
        print(content)
        print("Hello world")