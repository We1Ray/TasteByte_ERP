import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, waitFor, cleanup, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import React from "react";

// Mock next/navigation
const mockPush = vi.fn();
vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: mockPush,
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
  }),
  usePathname: () => "/login",
}));

// Mock next-intl - return key as-is for translation
vi.mock("next-intl", () => ({
  useTranslations: () => (key: string) => key,
}));

// Mock auth store
const mockSetAuth = vi.fn();
vi.mock("@/lib/stores/auth-store", () => ({
  useAuthStore: vi.fn((selector: (s: Record<string, unknown>) => unknown) =>
    selector({ setAuth: mockSetAuth })
  ),
}));

// Mock auth API
const mockLogin = vi.fn();
vi.mock("@/lib/api/auth", () => ({
  authApi: {
    login: (...args: unknown[]) => mockLogin(...args),
  },
}));

// Mock UI components to simplify testing
vi.mock("@/components/ui/button", () => ({
  Button: ({
    children,
    loading,
    ...props
  }: React.ButtonHTMLAttributes<HTMLButtonElement> & { loading?: boolean }) =>
    React.createElement(
      "button",
      { ...props, disabled: props.disabled || loading },
      loading ? "Loading..." : children
    ),
}));

vi.mock("@/components/ui/input", () => ({
  Input: React.forwardRef(
    (
      {
        label,
        error,
        ...props
      }: React.InputHTMLAttributes<HTMLInputElement> & {
        label?: string;
        error?: string;
      },
      ref: React.Ref<HTMLInputElement>
    ) =>
      React.createElement(
        "div",
        null,
        label &&
          React.createElement(
            "label",
            {
              htmlFor:
                props.id || label?.toLowerCase().replace(/\s+/g, "-"),
            },
            label
          ),
        React.createElement("input", {
          ref,
          id: props.id || label?.toLowerCase().replace(/\s+/g, "-"),
          "aria-label": label,
          ...props,
        }),
        error && React.createElement("span", { role: "alert" }, error)
      )
  ),
}));

// Mock zod/v4 - the login page uses this import path
vi.mock("zod/v4", async () => {
  const actual = await vi.importActual("zod");
  return actual;
});

import LoginPage from "@/app/login/page";

describe("LoginPage", () => {
  let container: HTMLElement;

  beforeEach(() => {
    cleanup();
    mockPush.mockClear();
    mockSetAuth.mockClear();
    mockLogin.mockClear();
  });

  afterEach(() => {
    cleanup();
  });

  function renderPage() {
    const result = render(React.createElement(LoginPage));
    container = result.container;
    return result;
  }

  it("renders username and password input fields", () => {
    renderPage();
    const view = within(container);

    expect(view.getByLabelText("username")).toBeInTheDocument();
    expect(view.getByLabelText("password")).toBeInTheDocument();
  });

  it("renders the sign in button", () => {
    renderPage();
    const view = within(container);

    expect(
      view.getByRole("button", { name: /loginButton/i })
    ).toBeInTheDocument();
  });

  it("renders the welcome text", () => {
    renderPage();
    const view = within(container);

    expect(view.getByText("welcomeBack")).toBeInTheDocument();
  });

  it("shows validation errors for empty fields on submit", async () => {
    const user = userEvent.setup();
    renderPage();
    const view = within(container);

    const submitButton = view.getByRole("button", { name: /loginButton/i });
    await user.click(submitButton);

    await waitFor(() => {
      const alerts = view.getAllByRole("alert");
      expect(alerts.length).toBeGreaterThanOrEqual(1);
    });
  });

  it("calls login API with form values on valid submit", async () => {
    mockLogin.mockResolvedValue({
      access_token: "tok",
      refresh_token: "ref",
      user: {
        id: "1",
        username: "admin",
        email: "a@b.c",
        roles: [],
        permissions: [],
      },
    });

    const user = userEvent.setup();
    renderPage();
    const view = within(container);

    await user.type(view.getByLabelText("username"), "admin");
    await user.type(view.getByLabelText("password"), "admin123");
    await user.click(view.getByRole("button", { name: /loginButton/i }));

    await waitFor(() => {
      expect(mockLogin).toHaveBeenCalledWith({
        username: "admin",
        password: "admin123",
      });
    });
  });

  it("stores auth and navigates to dashboard on successful login", async () => {
    const loginResponse = {
      access_token: "new-tok",
      refresh_token: "new-ref",
      user: {
        id: "1",
        username: "admin",
        email: "admin@test.com",
        display_name: "Admin",
        roles: ["ADMIN"],
        permissions: [],
      },
    };
    mockLogin.mockResolvedValue(loginResponse);

    const user = userEvent.setup();
    renderPage();
    const view = within(container);

    await user.type(view.getByLabelText("username"), "admin");
    await user.type(view.getByLabelText("password"), "admin123");
    await user.click(view.getByRole("button", { name: /loginButton/i }));

    await waitFor(() => {
      expect(mockSetAuth).toHaveBeenCalledWith(
        loginResponse.user,
        "new-tok",
        "new-ref"
      );
      expect(mockPush).toHaveBeenCalledWith("/dashboard");
    });
  });

  it("displays error message on login failure", async () => {
    mockLogin.mockRejectedValue(new Error("Invalid credentials"));

    const user = userEvent.setup();
    renderPage();
    const view = within(container);

    await user.type(view.getByLabelText("username"), "admin");
    await user.type(view.getByLabelText("password"), "wrongpass");
    await user.click(view.getByRole("button", { name: /loginButton/i }));

    await waitFor(() => {
      expect(view.getByText("loginError")).toBeInTheDocument();
    });
  });
});
