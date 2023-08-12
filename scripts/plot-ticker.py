import asyncio;
import information_theory as it;
import matplotlib.pyplot as plt
import numpy as np

async def main():
    (ticker_1, ticker_2) = await it.get_ticker_data_scatter("^NSEI", "000001.SS")
    time = np.arange(0, len(ticker_1)) * 100/len(ticker_1)
    plt.scatter(ticker_1, ticker_2, c=time)
    plt.show()

asyncio.run(main())