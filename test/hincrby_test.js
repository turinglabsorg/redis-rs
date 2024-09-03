const axios = require("axios");

const baseUrl = "http://0.0.0.0:3000";

async function testHIncrByEndpoint() {
  try {
    const response = await axios.post(`${baseUrl}/hincrby`, {
      key: "testHash",
      increment: 42,
      field: "incrField",
    });
    console.log("HINCRBY endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing HINCRBY endpoint:", error.message);
  }
}

async function testHGetEndpoint() {
  try {
    const response = await axios.get(`${baseUrl}/hget`, {
      params: {
        key: "testHash",
        field: "incrField",
      },
    });
    console.log("HGET endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing HGET endpoint:", error.message);
  }
}

async function runTests() {
  await testHIncrByEndpoint();
  await testHGetEndpoint();
}

runTests();
