"""Real HTTP stream regression: early bytes, large files, and truncation."""
import asyncio
import base64
import unittest
from relay_server import Stream, _write_resource_stream

class ResourceStreamTest(unittest.IsolatedAsyncioTestCase):
    async def run_stream(self, chunks, length=None, finish=True):
        stream = Stream(asyncio.Queue())
        release = asyncio.Event()
        async def serve(reader, writer):
            task = asyncio.create_task(_write_resource_stream(writer, stream, 200,
                [('content-type', 'application/octet-stream')] +
                ([] if length is None else [('content-length', str(length))])))
            stream.queue.put_nowait({'type':'http.response.chunk','data':base64.b64encode(chunks[0]).decode()})
            await release.wait()
            for chunk in chunks[1:]:
                await stream.queue.put({'type':'http.response.chunk','data':base64.b64encode(chunk).decode()})
            await stream.queue.put({'type':'http.response.end' if finish else 'error'})
            await task
        server=await asyncio.start_server(serve,'127.0.0.1',0)
        reader,writer=await asyncio.open_connection('127.0.0.1',server.sockets[0].getsockname()[1])
        try:
            head=await asyncio.wait_for(reader.readuntil(b'\r\n\r\n'),1)
            first=await asyncio.wait_for(reader.readexactly(len(chunks[0]) if length is not None else len(chunks[0])+5),1)
            # The producer has not finished, yet Mobile already has bytes.
            self.assertFalse(release.is_set())
            release.set()
            rest=await asyncio.wait_for(reader.read(),5)
            return head,first+rest
        finally:
            release.set();writer.close();await writer.wait_closed();server.close();await server.wait_closed()

    async def test_stream_above_old_eight_mib_limit(self):
        chunk=b'p'*32768
        head,body=await self.run_stream([chunk]*288,length=len(chunk)*288)
        self.assertIn(b'Content-Length: 9437184',head)
        self.assertEqual(len(body),9437184)

    async def test_old_desktop_without_length_uses_chunked(self):
        head,body=await self.run_stream([b'abc',b'def'])
        self.assertIn(b'Transfer-Encoding: chunked',head)
        self.assertEqual(body,b'3\r\nabc\r\n3\r\ndef\r\n0\r\n\r\n')

    async def test_upstream_error_does_not_finish_successfully(self):
        head,body=await self.run_stream([b'abc'],finish=False)
        self.assertNotIn(b'0\r\n\r\n',body)
