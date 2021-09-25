import json
import logging
import subprocess

log = logging.getLogger(__name__)


class Terraform:
    def __init__(self, workdir=None):
        self.workdir = workdir

    def apply(self):
        out = self._cmd('apply', '-auto-approve', '-no-color')

        return out

    def show(self):
        data = self._cmd('show', '-json', '-no-color')

        return json.loads(data)

    def _cmd(self, executeable, *args):
        cmd = ['terraform']
        if self.workdir:
            cmd.append(f'-chdir={self.workdir}')

        cmd.append(executeable)
        cmd.extend(args)

        proc = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE)
        stdout, stderr = proc.communicate()
        if stderr:
            log.error(stderr.decode('utf-8'))
            raise Exception(stderr.decode('utf-8'))

        return stdout.decode('utf-8')
