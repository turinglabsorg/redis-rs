const axios = require("axios");
const { fork } = require("child_process");

const BASE_URL = "http://localhost:3000";

async function stressTest() {
  const numRequests = 5000;
  const numProcesses = 16;
  const requestsPerProcess = Math.floor(numRequests / numProcesses);

  const processes = [];
  for (let i = 0; i < numProcesses; i++) {
    const process = fork("./stress_test_worker.js");
    processes.push(process);
  }

  const results = await Promise.all(
    processes.map((process) => {
      return new Promise((resolve) => {
        process.on("message", (result) => {
          resolve(result);
        });
        process.send({ baseUrl: BASE_URL, numRequests: requestsPerProcess });
      });
    })
  );

  const allRequestTimes = results.flat();
  const sortedTimes = allRequestTimes.sort((a, b) => a - b);
  const middleIndex = Math.floor(sortedTimes.length / 2);
  const medianTime = sortedTimes[middleIndex];

  console.log(
    `Stress test completed successfully. Median request time: ${medianTime} ms`
  );

  processes.forEach((process) => process.kill());
}

stressTest().catch((error) => {
  console.error("Stress test failed:", error);
});
