import { registerAction } from "./registry";

registerAction({
  id: "hello",
  name: "Say Hello",
  description: "A friendly greeting from the app",
  icon: "👋",
  category: "General",
  execute: async () => "Hello from Broc App!",
});
