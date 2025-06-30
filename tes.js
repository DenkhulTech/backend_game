// example.js
import http from "k6/http";
import { check } from "k6";

export const options = {
  scenarios: {
    high_rps: {
      executor: "constant-arrival-rate",
      rate: 5000, // ⬅️ target RPS
      timeUnit: "1s",
      duration: "1m",
      preAllocatedVUs: 500, // ⬅️ bisa naik jika bottleneck di client
      maxVUs: 5000,
    },
  },
};

export default function () {
  const res = http.get("http://localhost:3000/api/users"); // ganti sesuai endpoint kamu
  check(res, { "status 200": (r) => r.status === 200 });
}
