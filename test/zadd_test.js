const axios = require("axios");

const baseUrl = "http://0.0.0.0:3000";

async function testSetEndpoint() {
  try {
    const response = await axios.post(`${baseUrl}/set`, {
      key: "testKey",
      value: "testValue",
    });
    console.log("SET endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing SET endpoint:", error.message);
  }
}

async function testGetEndpoint() {
  try {
    const response = await axios.get(`${baseUrl}/get/testKey`);
    console.log("GET endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing GET endpoint:", error.message);
  }
}

async function testZAddEndpoint() {
  try {
    const response = await axios.post(`${baseUrl}/zadd`, {
      key: "testZSet",
      score: 4.0,
      member: "testMember",
    });
    console.log("ZADD endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing ZADD endpoint:", error.message);
  }
}

async function testZRangeEndpoint() {
  try {
    const response = await axios.get(`${baseUrl}/zrange`, {
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

async function runTests() {
  await testSetEndpoint();
  await testGetEndpoint();
  await testZAddEndpoint();
  await testZRangeEndpoint();
}

runTests();
