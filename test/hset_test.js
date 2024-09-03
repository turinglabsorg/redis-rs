const axios = require("axios");

const baseUrl = "http://0.0.0.0:3000";

async function testHSetEndpoint() {
  try {
    const response = await axios.post(`${baseUrl}/hset`, {
      key: "testHash",
      fields_and_values: [["incrField", 0]],
    });
    console.log("HSET endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing HSET endpoint:", error.message);
  }
}

async function testHGetEndpoint() {
  try {
    const response = await axios.get(`${baseUrl}/hgetall/testHash`);
    console.log("HGET endpoint response:", response.data);
  } catch (error) {
    console.error("Error testing HGET endpoint:", error.message);
  }
}

async function runTests() {
  await testHSetEndpoint();
  await testHGetEndpoint();
}

runTests();
