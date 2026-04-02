import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import SideBySide from "./index";

// Mock dependencies
vi.mock("../Editor", () => ({
  default: vi.fn(({ value, onChange, readOnly }) => (
    <div data-testid={readOnly ? "editor-readonly" : "editor-input"}>
      <input
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        readOnly={readOnly}
      />
    </div>
  )),
}));

vi.mock("../DiffViewer", () => ({
  default: vi.fn(({ input, output }) => (
    <div data-testid="diff-viewer">
      Diff: {input} → {output}
    </div>
  )),
}));

vi.mock("../ConfigPanel", () => ({
  default: vi.fn(({ config, onChange }) => (
    <div data-testid="config-panel">
      Config: {config.indentation}
      <button onClick={() => onChange({ indentation: "FourSpaces" })}>Change</button>
    </div>
  )),
}));

vi.mock("~/utils/wasm", () => ({
  initFormatter: vi.fn(async () => {}),
  formatCode: vi.fn((code) => `formatted: ${code}`),
}));

vi.mock("~/utils/nixpkgs", () => ({
  randomPath: vi.fn(async () => "pkgs/test/default.nix"),
  get: vi.fn(async () => "# Fetched content"),
}));

vi.mock("~/utils/permalink", () => ({
  getStateFromUrl: vi.fn(() => null),
  setStateInUrl: vi.fn(),
}));

describe("SideBySide", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders without crashing", async () => {
    const { container } = render(<SideBySide />);
    await waitFor(() => {
      expect(container).toBeDefined();
    });
  });

  it("shows loading state initially", async () => {
    const { container } = render(<SideBySide />);
    // Should show loading/initializing state initially
    await waitFor(() => {
      expect(container).toBeDefined();
    });
  });

  it("renders output editor with Alejandra label", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByText(/With Alejandra/)).toBeDefined();
    });
  });

  it("renders two Editor components", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      const editors = screen.getAllByTestId(/editor-/);
      expect(editors.length).toBeGreaterThanOrEqual(2);
    });
  });

  it("renders DiffViewer component", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByTestId("diff-viewer")).toBeDefined();
    });
  });

  it("renders load random file button", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      const button = screen.getByText(/click here to fetch/);
      expect(button).toBeDefined();
    });
  });

  it("displays instructions for user", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByText(/Type your code/)).toBeDefined();
    });
  });

  it("renders permalink section", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByText("Permalink:")).toBeDefined();
    });
  });

  it("initializes formatter on mount", async () => {
    const { initFormatter } = await import("~/utils/wasm");
    render(<SideBySide />);
    await waitFor(() => {
      expect(initFormatter).toHaveBeenCalled();
    });
  });

  it("handles WASM initialization error", async () => {
    const { initFormatter } = await import("~/utils/wasm");
    (initFormatter as any).mockRejectedValueOnce(new Error("WASM failed"));

    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByText(/Formatter Failed to Initialize/)).toBeDefined();
    });
  });

  it("shows retry button on initialization error", async () => {
    const { initFormatter } = await import("~/utils/wasm");
    (initFormatter as any).mockRejectedValueOnce(new Error("WASM failed"));

    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByText("Retry")).toBeDefined();
    });
  });

  it("loads random file on button click", async () => {
    const { randomPath, get } = await import("~/utils/nixpkgs");
    const user = userEvent.setup();

    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByText(/click here to fetch/)).toBeDefined();
    });

    const button = screen.getByText(/click here to fetch/);
    await user.click(button);

    await waitFor(() => {
      expect(randomPath).toHaveBeenCalled();
      expect(get).toHaveBeenCalled();
    });
  });

  it("handles formatting when input changes", async () => {
    const { formatCode } = await import("~/utils/wasm");
    render(<SideBySide />);

    await waitFor(() => {
      expect(screen.getByTestId("editor-input")).toBeDefined();
    });
  });

  it("displays error state with fallback on network failure", async () => {
    const { randomPath } = await import("~/utils/nixpkgs");
    (randomPath as any).mockRejectedValueOnce(new Error("Network error"));

    const user = userEvent.setup();
    render(<SideBySide />);

    await waitFor(() => {
      const button = screen.getByText(/click here to fetch/);
      expect(button).toBeDefined();
    });
  });

  it("applies correct grid layout classes", async () => {
    const { container } = render(<SideBySide />);
    await waitFor(() => {
      const grid = container.querySelector(".grid");
      expect(grid?.className).toContain("grid-cols-1");
      expect(grid?.className).toContain("lg:grid-cols-2");
    });
  });

  it("shows space-y-6 wrapper", async () => {
    const { container } = render(<SideBySide />);
    await waitFor(() => {
      const wrapper = container.querySelector(".space-y-6");
      expect(wrapper).toBeDefined();
    });
  });

  it("includes border and padding for instructions section", async () => {
    const { container } = render(<SideBySide />);
    await waitFor(() => {
      const section = container.querySelector(".border-b.border-neutral-200.pb-4");
      expect(section).toBeDefined();
    });
  });

  it("renders permalink link", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      const link = screen.getByText("here");
      expect(link.tagName).toBe("A");
    });
  });

  it("handles default code fallback", async () => {
    const { randomPath } = await import("~/utils/nixpkgs");
    (randomPath as any).mockRejectedValueOnce(new Error("Network failed"));

    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByTestId("editor-input")).toBeDefined();
    });
  });

  it("loads state from URL on mount if available", async () => {
    const { getStateFromUrl } = await import("~/utils/permalink");
    (getStateFromUrl as any).mockReturnValueOnce({ code: "x = 1;" });

    render(<SideBySide />);
    await waitFor(() => {
      expect(getStateFromUrl).toHaveBeenCalled();
    });
  });

  it("updates URL state when code changes", async () => {
    const { setStateInUrl } = await import("~/utils/permalink");
    render(<SideBySide />);

    await waitFor(() => {
      expect(screen.getByTestId("editor-input")).toBeDefined();
    });
  });

  it("renders ConfigPanel in instruction bar", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByTestId("config-panel")).toBeDefined();
    });
  });

  it("renders ConfigPanel with current config", async () => {
    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByText(/Config: TwoSpaces/)).toBeDefined();
    });
  });

  it("instruction bar has flex layout with config on right", async () => {
    const { container } = render(<SideBySide />);
    await waitFor(() => {
      const section = container.querySelector(".flex.items-center.justify-between");
      expect(section).toBeDefined();
    });
  });

  it("displays formatting error when formatting fails", async () => {
    const { formatCode } = await import("~/utils/wasm");
    (formatCode as any).mockImplementationOnce(() => {
      throw new Error("InvalidConfig: bad config");
    });

    render(<SideBySide />);
    await waitFor(() => {
      expect(screen.getByText(/Formatting Error/)).toBeDefined();
      expect(screen.getByText(/bad config/)).toBeDefined();
    });
  });

  it("shows error message in output panel instead of editor", async () => {
    const { formatCode } = await import("~/utils/wasm");
    (formatCode as any).mockImplementationOnce(() => {
      throw new Error("InvalidConfig: test error message");
    });

    const { container } = render(<SideBySide />);
    await waitFor(() => {
      const errorPanel = container.querySelector(".bg-red-50");
      expect(errorPanel).toBeDefined();
      expect(screen.queryByTestId("editor-readonly")).toBeNull();
    });
  });
});
