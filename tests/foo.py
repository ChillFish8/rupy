import asyncio


class TestException(Exception):
    pass


async def task():
    raise TestException('something went wrong')


async def init():
    def task_callback(tsk):
        print("foo")
        try:
            tsk.result()
        except TestException as e:
            raise e

    for i in range(1):
        t = asyncio.create_task(task())
        t.add_done_callback(task_callback)


loop = asyncio.get_event_loop()
loop.run_until_complete(init())
loop.run_forever()
