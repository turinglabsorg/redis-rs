const axios = require("axios");

process.on("message", async ({ baseUrl, numRequests }) => {
  const requestTimes = [];

  for (let i = 0; i < numRequests; i++) {
    const payload = {
      key: `key${i}`,
      value: `value${i}`,
    };
    const startTime = Date.now();
    await axios.post(`${baseUrl}/set`, payload);
    const endTime = Date.now();
    const requestTime = endTime - startTime;
    requestTimes.push(requestTime);
  }

  process.send(requestTimes);
});
