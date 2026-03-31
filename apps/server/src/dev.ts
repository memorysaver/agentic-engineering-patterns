import app from "./app";

const port = Number(process.env.PORT) || 3000;

export default {
  fetch: app.fetch,
  port,
};

console.log(`Server running at http://localhost:${port}`);
