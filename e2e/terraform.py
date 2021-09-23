import json
import logging
import subprocess

logger = logging.getLogger(__name__)

class Terraform:
    def __init__(self):
        pass

    def apply(self, workdir=None):
        stderr = subprocess.PIPE
        stdout = subprocess.PIPE

        p = subprocess.Popen(['terraform', f'-chdir={workdir}', 'apply', '-auto-approve', '-no-color'], stdout=stdout, stderr=stderr)
        out, err = p.communicate()
        logging.info(out)
        logging.error(err)
        # TODO: throw exception when error
        return out, err

    def show(self, workdir=None):
        stderr = subprocess.PIPE
        stdout = subprocess.PIPE

        p = subprocess.Popen(['terraform', f'-chdir={workdir}', 'show', '-json', '-no-color'], stdout=stdout, stderr=stderr)
        out, err = p.communicate()
        logging.info(out)
        logging.error(err)

        return json.loads(out)