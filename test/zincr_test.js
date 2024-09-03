const axios = require("axios");

const baseUrl = "http://0.0.0.0:3000";

async function testZIncrByEndpoint() {
  try {
    const response = await axios.post(`${baseUrl}/zincrby`, {
      key: "testZSet",
      increment: 338,
      member: "testMember",
    });
    console.log("ZINCRBY endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing ZINCRBY endpoint:", error.message);
  }
}

async function testZRangeEndpoint() {
  try {
    const response = await axios.get(`${baseUrl}/zrange_withscores`, {
      params: {
        key: "testZSet",
        start: 0,
        stop: -1,
      },
    });
    console.log("ZRANGE endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing ZRANGE endpoint:", error.message);
  }
}

async function testZRevRangeWithScoresEndpoint() {
  try {
    const response = await axios.get(`${baseUrl}/zrevrange_withscores`, {
      params: {
        key: "testZSet",
        start: 0,
        stop: -1,
      },
    });
    console.log("ZREVRANGE endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing ZREVRANGE endpoint:", error.message);
  }
}

async function runTests() {
  await testZIncrByEndpoint();
  await testZRangeEndpoint();
  await testZRevRangeWithScoresEndpoint();
}

runTests();
