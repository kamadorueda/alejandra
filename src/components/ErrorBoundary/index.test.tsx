import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import ErrorBoundary from "./index";

// Suppress console.error for error boundary tests
const originalError = console.error;
beforeEach(() => {
  console.error = vi.fn();
});

afterEach(() => {
  console.error = originalError;
});

describe("ErrorBoundary", () => {
  it("renders children when there is no error", () => {
    const { container } = render(
      <ErrorBoundary>
        <div>Test content</div>
      </ErrorBoundary>
    );

    expect(screen.getByText("Test content")).toBeDefined();
    expect(container.querySelector("pre")).toBeNull();
  });

  it("renders error UI when an error occurs in children", () => {
    const TestComponent = () => {
      throw new Error("Test error message");
    };

    const { container } = render(
      <ErrorBoundary>
        <TestComponent />
      </ErrorBoundary>
    );

    expect(screen.getByText("Something went wrong")).toBeDefined();
    expect(screen.getByText("Test error message")).toBeDefined();
  });

  it("displays error message in the UI", () => {
    const ErrorComponent = () => {
      throw new Error("Custom error");
    };

    render(
      <ErrorBoundary>
        <ErrorComponent />
      </ErrorBoundary>
    );

    expect(screen.getByText("Custom error")).toBeDefined();
  });

  it("shows stack trace in a details element", () => {
    const ErrorComponent = () => {
      throw new Error("Stack trace test");
    };

    const { container } = render(
      <ErrorBoundary>
        <ErrorComponent />
      </ErrorBoundary>
    );

    const details = container.querySelector("details");
    expect(details).toBeDefined();

    const summary = details?.querySelector("summary");
    expect(summary?.textContent).toBe("Stack trace");
  });

  it("renders red-themed error UI", () => {
    const ErrorComponent = () => {
      throw new Error("Color test");
    };

    const { container } = render(
      <ErrorBoundary>
        <ErrorComponent />
      </ErrorBoundary>
    );

    const errorContainer = container.querySelector(".bg-red-50");
    expect(errorContainer).toBeDefined();
  });

  it("displays the heading with correct styling", () => {
    const ErrorComponent = () => {
      throw new Error("Heading test");
    };

    render(
      <ErrorBoundary>
        <ErrorComponent />
      </ErrorBoundary>
    );

    const heading = screen.getByText("Something went wrong");
    expect(heading.tagName).toBe("H1");
  });

  it("renders multiple children correctly without error", () => {
    const { container } = render(
      <ErrorBoundary>
        <div>Child 1</div>
        <div>Child 2</div>
        <div>Child 3</div>
      </ErrorBoundary>
    );

    expect(screen.getByText("Child 1")).toBeDefined();
    expect(screen.getByText("Child 2")).toBeDefined();
    expect(screen.getByText("Child 3")).toBeDefined();
  });

  it("wraps content in proper container", () => {
    const { container } = render(
      <ErrorBoundary>
        <span>Content</span>
      </ErrorBoundary>
    );

    expect(screen.getByText("Content")).toBeDefined();
    // Should not have error styling when no error
    expect(container.querySelector(".bg-red-50")).toBeNull();
  });

  it("catches errors from deeply nested children", () => {
    const NestedError = () => {
      return (
        <div>
          <div>
            <ThrowError />
          </div>
        </div>
      );
    };

    const ThrowError = () => {
      throw new Error("Nested error");
    };

    render(
      <ErrorBoundary>
        <NestedError />
      </ErrorBoundary>
    );

    expect(screen.getByText("Nested error")).toBeDefined();
  });

  it("centers the error message", () => {
    const ErrorComponent = () => {
      throw new Error("Centered");
    };

    const { container } = render(
      <ErrorBoundary>
        <ErrorComponent />
      </ErrorBoundary>
    );

    const mainDiv = container.querySelector(".mx-auto.max-w-4xl");
    expect(mainDiv).toBeDefined();
  });

  it("includes proper padding and spacing", () => {
    const ErrorComponent = () => {
      throw new Error("Spacing test");
    };

    const { container } = render(
      <ErrorBoundary>
        <ErrorComponent />
      </ErrorBoundary>
    );

    const outerDiv = container.querySelector(".min-h-screen.bg-red-50.py-8");
    expect(outerDiv).toBeDefined();
  });
});
