import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import React from "react";

// Mock next/navigation
let mockPathname = "/dashboard";
vi.mock("next/navigation", () => ({
  usePathname: () => mockPathname,
}));

// Mock next/link
vi.mock("next/link", () => ({
  default: ({
    children,
    href,
    ...props
  }: React.PropsWithChildren<{ href: string }>) =>
    React.createElement("a", { href, ...props }, children),
}));

// Mock next-intl - return key as-is
vi.mock("next-intl", () => ({
  useTranslations: () => (key: string) => key,
}));

// Mock lucide-react icons
vi.mock("lucide-react", () => {
  const iconNames = [
    "LayoutDashboard",
    "DollarSign",
    "PieChart",
    "Package",
    "ShoppingCart",
    "Factory",
    "Users",
    "Warehouse",
    "ClipboardCheck",
    "ChevronLeft",
    "ChevronRight",
    "Blocks",
    "Code2",
    "Settings",
  ];
  const icons: Record<string, React.FC<{ className?: string }>> = {};
  for (const name of iconNames) {
    icons[name] = ({ className }: { className?: string }) =>
      React.createElement("svg", {
        "data-testid": `icon-${name}`,
        className,
      });
  }
  return icons;
});

// Store mocks
const mockToggleSidebarCollapsed = vi.fn();
let mockSidebarCollapsed = false;

vi.mock("@/lib/stores/ui-store", () => ({
  useUiStore: vi.fn(() => ({
    sidebarCollapsed: mockSidebarCollapsed,
    toggleSidebarCollapsed: mockToggleSidebarCollapsed,
  })),
}));

let mockUser = {
  id: "1",
  username: "admin",
  email: "admin@test.com",
  display_name: "Admin",
  roles: ["ADMIN"] as string[],
  permissions: [
    "fi:read",
    "co:read",
    "mm:read",
    "sd:read",
    "pp:read",
    "hr:read",
    "wm:read",
    "qm:read",
  ],
};

vi.mock("@/lib/stores/auth-store", () => ({
  useAuthStore: vi.fn((selector: (s: Record<string, unknown>) => unknown) =>
    selector({ user: mockUser })
  ),
}));

vi.mock("@/lib/hooks/use-platform-role", () => ({
  usePlatformRole: vi.fn(() => ({
    roles: ["PLATFORM_ADMIN"],
    isAdmin: true,
    isDeveloper: true,
    isUser: true,
    projects: [],
    isLoading: false,
    isTraditionalAdmin: true,
  })),
}));

vi.mock("@/lib/utils", () => ({
  cn: (...args: unknown[]) => args.filter(Boolean).join(" "),
}));

import { ErpSidebar } from "@/components/layout/erp-sidebar";
import { usePlatformRole } from "@/lib/hooks/use-platform-role";

describe("ErpSidebar", () => {
  let container: HTMLElement;

  function renderSidebar() {
    const result = render(React.createElement(ErpSidebar));
    container = result.container;
    return result;
  }

  beforeEach(() => {
    cleanup();
    mockPathname = "/dashboard";
    mockSidebarCollapsed = false;
    mockToggleSidebarCollapsed.mockClear();
    mockUser = {
      id: "1",
      username: "admin",
      email: "admin@test.com",
      display_name: "Admin",
      roles: ["ADMIN"],
      permissions: [
        "fi:read",
        "co:read",
        "mm:read",
        "sd:read",
        "pp:read",
        "hr:read",
        "wm:read",
        "qm:read",
      ],
    };
    (usePlatformRole as ReturnType<typeof vi.fn>).mockReturnValue({
      roles: ["PLATFORM_ADMIN"],
      isAdmin: true,
      isDeveloper: true,
      isUser: true,
      projects: [],
      isLoading: false,
      isTraditionalAdmin: true,
    });
  });

  afterEach(() => {
    cleanup();
  });

  it("renders the TasteByte brand link", () => {
    renderSidebar();
    const view = within(container);

    expect(view.getByText("TasteByte")).toBeInTheDocument();
  });

  it("renders navigation links for all modules when user is admin", () => {
    renderSidebar();
    const view = within(container);

    // Translation returns key as-is; sidebar renders shortLabel ? `${shortLabel} - ${label}` : label
    expect(view.getByText("dashboard")).toBeInTheDocument();
    expect(view.getByText(/FI - fi/)).toBeInTheDocument();
    expect(view.getByText(/CO - co/)).toBeInTheDocument();
    expect(view.getByText(/MM - mm/)).toBeInTheDocument();
    expect(view.getByText(/SD - sd/)).toBeInTheDocument();
    expect(view.getByText(/PP - pp/)).toBeInTheDocument();
    expect(view.getByText(/HR - hr/)).toBeInTheDocument();
    expect(view.getByText(/WM - wm/)).toBeInTheDocument();
    expect(view.getByText(/QM - qm/)).toBeInTheDocument();
  });

  it("renders platform sections for admin users", () => {
    renderSidebar();
    const view = within(container);

    expect(view.getByText("lowcode")).toBeInTheDocument();
    expect(view.getByText("developer")).toBeInTheDocument();
    expect(view.getByText("admin")).toBeInTheDocument();
  });

  it("renders version footer", () => {
    renderSidebar();
    const view = within(container);

    expect(view.getByText("TasteByte ERP v1.0")).toBeInTheDocument();
  });

  it("calls toggleSidebarCollapsed when collapse button is clicked", async () => {
    const user = userEvent.setup();
    renderSidebar();
    const view = within(container);

    const collapseButton = view
      .getByTestId("icon-ChevronLeft")
      .closest("button")!;
    await user.click(collapseButton);

    expect(mockToggleSidebarCollapsed).toHaveBeenCalledTimes(1);
  });

  describe("permission-based filtering", () => {
    it("hides modules when user lacks required permissions and is not admin", () => {
      mockUser = {
        id: "2",
        username: "limited",
        email: "limited@test.com",
        display_name: "Limited User",
        roles: ["USER"],
        permissions: ["fi:read"],
      };

      (usePlatformRole as ReturnType<typeof vi.fn>).mockReturnValue({
        roles: [],
        isAdmin: false,
        isDeveloper: false,
        isUser: false,
        projects: [],
        isLoading: false,
        isTraditionalAdmin: false,
      });

      renderSidebar();
      const view = within(container);

      expect(view.getByText("dashboard")).toBeInTheDocument();
      expect(view.getByText(/FI - fi/)).toBeInTheDocument();

      // MM should be hidden (user lacks mm:read)
      expect(view.queryByText(/MM - mm/)).not.toBeInTheDocument();
      expect(view.queryByText(/SD - sd/)).not.toBeInTheDocument();

      // Platform sections should be hidden
      expect(view.queryByText("admin")).not.toBeInTheDocument();
      expect(view.queryByText("developer")).not.toBeInTheDocument();
      expect(view.queryByText("lowcode")).not.toBeInTheDocument();
    });

    it("shows all ERP modules for ADMIN role (bypasses permission check)", () => {
      mockUser = {
        id: "1",
        username: "admin",
        email: "admin@test.com",
        display_name: "Admin",
        roles: ["ADMIN"],
        permissions: [],
      };

      (usePlatformRole as ReturnType<typeof vi.fn>).mockReturnValue({
        roles: [],
        isAdmin: false,
        isDeveloper: false,
        isUser: false,
        projects: [],
        isLoading: false,
        isTraditionalAdmin: false,
      });

      renderSidebar();
      const view = within(container);

      expect(view.getByText(/FI - fi/)).toBeInTheDocument();
      expect(view.getByText(/MM - mm/)).toBeInTheDocument();
      expect(view.getByText(/SD - sd/)).toBeInTheDocument();
      expect(view.getByText(/HR - hr/)).toBeInTheDocument();
    });

    it("hides Developer Mode when user is not a developer", () => {
      mockUser = {
        id: "3",
        username: "basicuser",
        email: "basic@test.com",
        display_name: "Basic",
        roles: ["USER"],
        permissions: ["fi:read"],
      };

      (usePlatformRole as ReturnType<typeof vi.fn>).mockReturnValue({
        roles: ["USER"],
        isAdmin: false,
        isDeveloper: false,
        isUser: true,
        projects: [],
        isLoading: false,
        isTraditionalAdmin: false,
      });

      renderSidebar();
      const view = within(container);

      expect(view.queryByText("developer")).not.toBeInTheDocument();
      expect(view.getByText("lowcode")).toBeInTheDocument();
    });
  });

  describe("active state", () => {
    it("highlights the active module based on pathname", () => {
      mockPathname = "/fi/accounts";
      renderSidebar();
      const view = within(container);

      const fiLink = view.getByText(/FI - fi/).closest("a")!;
      expect(fiLink.className).toContain("blue");
    });

    it("shows child links when parent module is active", () => {
      mockPathname = "/fi/accounts";
      renderSidebar();
      const view = within(container);

      expect(view.getByText("chartOfAccounts")).toBeInTheDocument();
      expect(view.getByText("journalEntries")).toBeInTheDocument();
      expect(view.getByText("reports")).toBeInTheDocument();
    });

    it("does not show child links for inactive modules", () => {
      mockPathname = "/dashboard";
      renderSidebar();
      const view = within(container);

      expect(view.queryByText("chartOfAccounts")).not.toBeInTheDocument();
      expect(view.queryByText("journalEntries")).not.toBeInTheDocument();
    });
  });
});
