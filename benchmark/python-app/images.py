import asyncio
import uuid
from datetime import datetime


class Image:
    def __init__(self):
        self.uuid = self._generate_uuid()
        self.last_modified = self._get_time()

    def _generate_uuid(self):
        return str(uuid.uuid4())

    def _get_time(self):
        return datetime.now()


async def download():
    await asyncio.sleep(0.2)


async def save(image: Image):
    await asyncio.sleep(0.5)
    del image
