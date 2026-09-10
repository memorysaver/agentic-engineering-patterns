import { Link, useRouterState } from "@tanstack/react-router";
import { Map, LayoutDashboard, GitBranch, BarChart3 } from "lucide-react";

const navItems = [
  { to: "/story-map", label: "Story Map", icon: Map },
  { to: "/overview", label: "Overview", icon: LayoutDashboard },
  { to: "/progress", label: "Progress", icon: BarChart3 },
  { to: "/architecture", label: "Architecture", icon: GitBranch },
] as const;

export function AppSidebar() {
  const routerState = useRouterState();
  const currentPath = routerState.location.pathname;

  return (
    <aside className="flex w-56 flex-col border-r bg-card">
      <div className="border-b p-4">
        <h1 className="text-sm font-bold tracking-tight">AEP Dashboard</h1>
        <p className="text-muted-foreground text-xs">Product Context Viewer</p>
      </div>
      <nav className="flex-1 p-2">
        {navItems.map((item) => {
          const isActive = currentPath === item.to;
          return (
            <Link
              key={item.to}
              to={item.to}
              className={`flex items-center gap-2 rounded-md px-3 py-2 text-sm transition-colors ${
                isActive
                  ? "bg-accent text-accent-foreground font-medium"
                  : "text-muted-foreground hover:bg-accent/50 hover:text-foreground"
              }`}
            >
              <item.icon className="h-4 w-4" />
              {item.label}
            </Link>
          );
        })}
      </nav>
      <div className="border-t p-3">
        <p className="text-muted-foreground text-xs">Local dev mode</p>
      </div>
    </aside>
  );
}
