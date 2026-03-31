import { BrowserWindow, Updater } from "electrobun/bun";

const DEV_SERVER_PORT = 3001;
const DEV_SERVER_URL = `http://localhost:${DEV_SERVER_PORT}`;

// Check if the web dev server is running for HMR
async function getMainViewUrl(): Promise<string> {
  const channel = await Updater.localInfo.channel();
  if (channel === "dev") {
    try {
      await fetch(DEV_SERVER_URL, { method: "HEAD" });
      console.log(`HMR enabled: Using web dev server at ${DEV_SERVER_URL}`);
      return DEV_SERVER_URL;
    } catch {
      console.log('Web dev server not running. Run "bun run dev:hmr" for HMR support.');
    }
  }

  return "views://mainview/index.html";
}

const url = await getMainViewUrl();

new BrowserWindow({
  title: "agentic-engineering-patterns",
  url,
  frame: {
    width: 1280,
    height: 820,
    x: 120,
    y: 120,
  },
});

console.log("Electrobun desktop shell started.");
