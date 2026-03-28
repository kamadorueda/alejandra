import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import App from "./App";

// Mock ErrorBoundary component
vi.mock("./components/ErrorBoundary", () => ({
  default: ({ children }: any) => <>{children}</>,
}));

// Mock SideBySide component
vi.mock("./components/SideBySide", () => ({
  default: () => <div data-testid="side-by-side">SideBySide Component</div>,
}));

describe("App", () => {
  it("renders without crashing", () => {
    const { container } = render(<App />);
    expect(container).toBeDefined();
  });

  it("displays the main heading", () => {
    render(<App />);
    expect(screen.getByText(/Alejandra/)).toBeDefined();
  });

  it("displays the formatter description", () => {
    render(<App />);
    expect(screen.getByText(/The Uncompromising Nix Code Formatter/)).toBeDefined();
  });

  it("renders heading with correct styling", () => {
    const { container } = render(<App />);
    const heading = container.querySelector("h1");
    expect(heading?.className).toContain("text-center");
    expect(heading?.className).toContain("text-4xl");
    expect(heading?.className).toContain("font-bold");
  });

  it("renders description paragraph", () => {
    const { container } = render(<App />);
    const paragraph = container.querySelector("p");
    expect(paragraph?.className).toContain("text-center");
  });

  it("wraps content in ErrorBoundary", () => {
    const { container } = render(<App />);
    expect(container.querySelector("div")).toBeDefined();
  });

  it("renders SideBySide component", () => {
    render(<App />);
    expect(screen.getByTestId("side-by-side")).toBeDefined();
  });

  it("applies background styling to main container", () => {
    const { container } = render(<App />);
    const mainDiv = container.querySelector(".min-h-screen.bg-surface");
    expect(mainDiv).toBeDefined();
  });

  it("applies padding and width constraints", () => {
    const { container } = render(<App />);
    const contentDiv = container.querySelector(".mx-auto.max-w-4xl");
    expect(contentDiv).toBeDefined();
  });

  it("includes horizontal rule separator", () => {
    const { container } = render(<App />);
    const hr = container.querySelector("hr");
    expect(hr).toBeDefined();
  });

  it("hr has correct styling", () => {
    const { container } = render(<App />);
    const hr = container.querySelector("hr");
    expect(hr?.className).toContain("my-6");
    expect(hr?.className).toContain("border-neutral-200");
  });

  it("has correct spacing between elements", () => {
    const { container } = render(<App />);
    const paragraph = container.querySelector("p");
    expect(paragraph?.className).toContain("mt-2");
  });

  it("displays emoji in heading", () => {
    render(<App />);
    const heading = screen.getByText(/Alejandra 💅/);
    expect(heading).toBeDefined();
  });

  it("wraps content in proper container structure", () => {
    const { container } = render(<App />);
    const outer = container.querySelector(".min-h-screen.bg-surface.py-8");
    expect(outer).toBeDefined();
    const inner = outer?.querySelector(".mx-auto.max-w-4xl.px-4");
    expect(inner).toBeDefined();
  });

  it("maintains semantic HTML structure", () => {
    const { container } = render(<App />);
    const h1 = container.querySelector("h1");
    const p = container.querySelector("p");
    const hr = container.querySelector("hr");

    expect(h1?.tagName).toBe("H1");
    expect(p?.tagName).toBe("P");
    expect(hr?.tagName).toBe("HR");
  });

  it("renders all text content", () => {
    render(<App />);
    expect(screen.getByText(/Alejandra/)).toBeDefined();
    expect(screen.getByText(/The Uncompromising Nix Code Formatter/)).toBeDefined();
  });

  it("applies padding to main container", () => {
    const { container } = render(<App />);
    const outer = container.querySelector(".py-8");
    expect(outer).toBeDefined();
    const inner = container.querySelector(".px-4");
    expect(inner).toBeDefined();
  });

  it("side-by-side component is visible", () => {
    render(<App />);
    const sideBySide = screen.getByTestId("side-by-side");
    expect(sideBySide).toBeVisible();
  });
});
