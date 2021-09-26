const request = require("supertest");
const server = require("../index");

afterEach((done) => {
  server.close(done);
});

test("GET /node should return 200 status", async () => {
  await request(server).get("/node").expect(200);
});