import subprocess
from pathlib import Path

class WrapVT100:
    def __init__(self):
        self.bin = subprocess.Popen(
                [f'{Path.home()}/.cargo/bin/wrap-vt100'],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE
                )

    def wrap(self, line):
        if not line.endswith('\n'):
            line += '\n'

        self.bin.stdin.write(line.encode())
        self.bin.stdin.flush()

        return self.bin.stdout.readline().decode()
