import time


def wait_for_status(func, timeout=60, status='up', step=1):
    with Timer(timeout=timeout) as timer:
        while not timer.passed():
            if func() == status:
                return True
            time.sleep(step)

    return False


class Timer:
    def __init__(self, timeout=60):
        self.timeout = timeout

    def __enter__(self):
        self.start = time.time()
        return self

    def __exit__(self, *args):
        pass

    def passed(self):
        return (time.time() - self.start) > self.timeout
