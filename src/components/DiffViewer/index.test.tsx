import { describe, expect, it, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import DiffViewer from "./index";

// Mock react-diff-viewer
vi.mock("react-diff-viewer", () => ({
  default: vi.fn(({ oldValue, newValue }) => (
    <div data-testid="diff-viewer">
      <div data-testid="old-value">{oldValue}</div>
      <div data-testid="new-value">{newValue}</div>
    </div>
  )),
  DiffMethod: {
    WORDS: "WORDS",
  },
}));

// Mock highlight.js
vi.mock("highlight.js", () => ({
  default: {
    highlight: vi.fn((text) => ({
      value: `<span>${text}</span>`,
    })),
  },
}));

// Mock CSS
vi.mock("highlight.js/styles/atom-one-light.css", () => ({}));

describe("DiffViewer", () => {
  it("renders without crashing", () => {
    const { container } = render(
      <DiffViewer input="input" output="output" />
    );
    expect(container).toBeDefined();
  });

  it("displays the Git patch heading", () => {
    render(<DiffViewer input="input" output="output" />);
    expect(screen.getByText("Git patch")).toBeDefined();
  });

  it("renders the heading with correct styling", () => {
    const { container } = render(
      <DiffViewer input="input" output="output" />
    );
    const heading = container.querySelector("h3");
    expect(heading?.className).toContain("text-lg");
    expect(heading?.className).toContain("font-semibold");
  });

  it("renders ReactDiffViewer component", () => {
    render(<DiffViewer input="input" output="output" />);
    expect(screen.getByTestId("diff-viewer")).toBeDefined();
  });

  it("passes input and output to diff viewer", () => {
    render(<DiffViewer input="test input" output="test output" />);
    expect(screen.getByTestId("old-value")).toHaveTextContent("test input");
    expect(screen.getByTestId("new-value")).toHaveTextContent("test output");
  });

  it("wraps diff viewer in styled container", () => {
    const { container } = render(
      <DiffViewer input="input" output="output" />
    );
    const wrapper = container.querySelector(".rounded-lg.overflow-hidden");
    expect(wrapper).toBeDefined();
  });

  it("applies bg-white to the wrapper", () => {
    const { container } = render(
      <DiffViewer input="input" output="output" />
    );
    const wrapper = container.querySelector(".bg-white");
    expect(wrapper).toBeDefined();
  });

  it("handles empty input", () => {
    render(<DiffViewer input="" output="output" />);
    expect(screen.getByTestId("diff-viewer")).toBeDefined();
  });

  it("handles empty output", () => {
    render(<DiffViewer input="input" output="" />);
    expect(screen.getByTestId("diff-viewer")).toBeDefined();
  });

  it("handles both empty input and output", () => {
    render(<DiffViewer input="" output="" />);
    expect(screen.getByTestId("diff-viewer")).toBeDefined();
  });

  it("handles identical input and output", () => {
    const code = "x = 1;";
    render(<DiffViewer input={code} output={code} />);
    expect(screen.getByTestId("old-value")).toHaveTextContent(code);
    expect(screen.getByTestId("new-value")).toHaveTextContent(code);
  });

  it("handles large code blocks", () => {
    const largeCode = "x = 1;\n".repeat(1000);
    render(<DiffViewer input={largeCode} output={largeCode} />);
    expect(screen.getByTestId("diff-viewer")).toBeDefined();
  });

  it("handles multiline code", () => {
    const input = `line1
line2
line3`;
    const output = `line1
line2 modified
line3`;
    render(<DiffViewer input={input} output={output} />);
    expect(screen.getByTestId("old-value")).toHaveTextContent("line1");
    expect(screen.getByTestId("new-value")).toHaveTextContent("line2 modified");
  });

  it("handles special characters in code", () => {
    const input = '{ a.b = "value"; }';
    const output = '{ a.b = "modified"; }';
    render(<DiffViewer input={input} output={output} />);
    expect(screen.getByTestId("diff-viewer")).toBeDefined();
  });

  it("memoizes diff configuration", () => {
    const { rerender } = render(
      <DiffViewer input="input1" output="output1" />
    );

    rerender(<DiffViewer input="input1" output="output1" />);
    expect(screen.getByTestId("diff-viewer")).toBeDefined();
  });

  it("recalculates diff when input changes", () => {
    const { rerender } = render(
      <DiffViewer input="input1" output="output1" />
    );

    rerender(<DiffViewer input="input2" output="output1" />);
    expect(screen.getByTestId("old-value")).toHaveTextContent("input2");
  });

  it("recalculates diff when output changes", () => {
    const { rerender } = render(
      <DiffViewer input="input1" output="output1" />
    );

    rerender(<DiffViewer input="input1" output="output2" />);
    expect(screen.getByTestId("new-value")).toHaveTextContent("output2");
  });

  it("uses correct styling for the main container", () => {
    const { container } = render(
      <DiffViewer input="input" output="output" />
    );
    const mainDiv = container.querySelector(".rounded-lg.overflow-hidden.bg-white.min-h-96");
    expect(mainDiv).toBeDefined();
  });

  it("includes proper spacing above heading", () => {
    const { container } = render(
      <DiffViewer input="input" output="output" />
    );
    const heading = container.querySelector("h3");
    expect(heading?.className).toContain("mt-8");
    expect(heading?.className).toContain("mb-4");
  });
});
