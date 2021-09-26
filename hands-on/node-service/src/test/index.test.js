const request = require("supertest");
const server = require("../index");

afterEach((done) => {
  server.close(done);
});

test("GET / should return 200 status", async () => {
  await request(server).get("/").expect(201);
});