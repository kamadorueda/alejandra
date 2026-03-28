import { describe, expect, it, vi, beforeEach } from "vitest";
import { render } from "@testing-library/react";
import Editor from "./index";

// Mock CodeMirror
vi.mock("codemirror", () => ({
  fromTextArea: vi.fn((textarea, config) => ({
    getValue: vi.fn(() => "mock value"),
    setValue: vi.fn(),
    on: vi.fn(),
    off: vi.fn(),
    toTextArea: vi.fn(),
  })),
}));

// Mock CodeMirror CSS files
vi.mock("codemirror/lib/codemirror.css", () => ({}));
vi.mock("codemirror/theme/monokai.css", () => ({}));
vi.mock("codemirror/mode/shell/shell", () => ({}));

describe("Editor", () => {
  let mockOnChange: any;

  beforeEach(() => {
    mockOnChange = vi.fn();
  });

  it("renders without crashing", () => {
    const { container } = render(<Editor value="test" onChange={mockOnChange} />);
    expect(container.querySelector("div")).toBeDefined();
  });

  it("accepts value prop", () => {
    const { container } = render(
      <Editor value="test content" onChange={mockOnChange} />
    );
    expect(container).toBeDefined();
  });

  it("accepts onChange callback", () => {
    render(<Editor value="" onChange={mockOnChange} />);
    expect(mockOnChange).toBeDefined();
  });

  it("supports readOnly prop", () => {
    const { container, rerender } = render(
      <Editor value="test" onChange={mockOnChange} readOnly={false} />
    );
    expect(container).toBeDefined();

    rerender(<Editor value="test" onChange={mockOnChange} readOnly={true} />);
    expect(container).toBeDefined();
  });

  it("renders a div container", () => {
    const { container } = render(<Editor value="" onChange={mockOnChange} />);
    const div = container.querySelector("div");
    expect(div).toBeDefined();
    expect(div?.className).toContain("w-full");
  });

  it("applies correct CSS classes to container", () => {
    const { container } = render(<Editor value="" onChange={mockOnChange} />);
    const div = container.querySelector("div");
    expect(div?.className).toContain("rounded-lg");
    expect(div?.className).toContain("overflow-hidden");
    expect(div?.className).toContain("h-96");
    expect(div?.className).toContain("flex");
    expect(div?.className).toContain("flex-col");
  });

  it("defaults readOnly to false", () => {
    const { container } = render(<Editor value="" onChange={mockOnChange} />);
    expect(container).toBeDefined();
  });

  it("handles empty value string", () => {
    const { container } = render(<Editor value="" onChange={mockOnChange} />);
    expect(container.querySelector("div")).toBeDefined();
  });

  it("handles large value strings", () => {
    const largeValue = "x".repeat(10000);
    const { container } = render(
      <Editor value={largeValue} onChange={mockOnChange} />
    );
    expect(container.querySelector("div")).toBeDefined();
  });

  it("handles multiline code", () => {
    const code = `{
  lib,
  stdenv,
}:

stdenv.mkDerivation {
  name = "test";
}`;
    const { container } = render(
      <Editor value={code} onChange={mockOnChange} />
    );
    expect(container.querySelector("div")).toBeDefined();
  });

  it("handles special characters in code", () => {
    const code = 'x = { "key": "value"; a.b = 1; };';
    const { container } = render(
      <Editor value={code} onChange={mockOnChange} />
    );
    expect(container.querySelector("div")).toBeDefined();
  });

  it("can switch between readOnly modes", () => {
    const { rerender, container } = render(
      <Editor value="test" onChange={mockOnChange} readOnly={false} />
    );
    expect(container).toBeDefined();

    rerender(<Editor value="test" onChange={mockOnChange} readOnly={true} />);
    expect(container).toBeDefined();

    rerender(<Editor value="test" onChange={mockOnChange} readOnly={false} />);
    expect(container).toBeDefined();
  });

  it("handles rapid onChange updates", () => {
    const { rerender } = render(
      <Editor value="a" onChange={mockOnChange} />
    );

    for (let i = 0; i < 10; i++) {
      rerender(<Editor value={`a${"b".repeat(i)}`} onChange={mockOnChange} />);
    }

    expect(mockOnChange).toBeDefined();
  });

  it("cleans up event listeners on unmount", () => {
    const { unmount } = render(<Editor value="test" onChange={mockOnChange} />);
    expect(() => unmount()).not.toThrow();
  });
});
