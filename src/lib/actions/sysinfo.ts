import { registerAction, backendAction } from "./registry";

registerAction(
  backendAction({
    id: "sysinfo",
    name: "System Info",
    description: "Shows OS, hostname, and architecture",
    icon: "🖥️",
    category: "System",
    command: "get_sysinfo",
  })
);
