const express = require("express");
require("dotenv").config();

const app = express();
const env = process.env.ENV || "dev";
const port = process.env.PORT || 3000;

app.get("/", (req, res) => {
  return res.json({
    message: `Node Service on env : ${env}`,
  });
});

const server = app.listen(port, () =>
  console.log(`Node Service start on port ==> ${port} | env : ${env}`)
);

module.exports = server;