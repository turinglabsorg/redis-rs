import http from "k6/http";

export const options = {
  vus: 100,
  duration: "120s",
  cloud: {
    // Project: Default project
    projectID: 3711743,
    // Test runs with the same name groups test runs together.
    name: "Test " + new Date().getTime(),
  },
};

export default function () {
  const data = {
    key: `testredis:key${new Date().getTime().toString()}`,
    value: new Date().getTime().toString(),
  };

  // Using a JSON string as body
  const url = "<LIVE_URL_ENPOINT>/set";
  let res = http.post(url, JSON.stringify(data), {
    headers: { "Content-Type": "application/json" },
  });
  console.log(res.json().json.name); // Bert

  // Using an object as body, the headers will automatically include
  // 'Content-Type: application/x-www-form-urlencoded'.
  res = http.post(url, data);
  console.log(res.json().form.name);
}
