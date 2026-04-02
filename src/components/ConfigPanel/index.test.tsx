import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import ConfigPanel from "./index";
import { DEFAULT_CONFIG } from "~/types/config";

describe("ConfigPanel", () => {
  it("renders without crashing", () => {
    const mockOnChange = vi.fn();
    render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);
    expect(screen.getByText("Indentation:")).toBeDefined();
  });

  it("renders select dropdown", () => {
    const mockOnChange = vi.fn();
    const { container } = render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);
    const select = container.querySelector("select");
    expect(select).toBeDefined();
  });

  it("displays label for indentation", () => {
    const mockOnChange = vi.fn();
    render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);
    expect(screen.getByText("Indentation:")).toBeDefined();
  });

  it("shows all three indentation options", () => {
    const mockOnChange = vi.fn();
    const { container } = render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);
    const options = container.querySelectorAll("option");
    expect(options.length).toBe(3);
    expect(options[0].textContent).toBe("2 Spaces");
    expect(options[1].textContent).toBe("4 Spaces");
    expect(options[2].textContent).toBe("Tabs");
  });

  it("displays current config value as selected option", () => {
    const mockOnChange = vi.fn();
    const config = { indentation: "FourSpaces" as const };
    const { container } = render(<ConfigPanel config={config} onChange={mockOnChange} />);
    const select = container.querySelector("select") as HTMLSelectElement;
    expect(select.value).toBe("FourSpaces");
  });

  it("calls onChange when selection changes to FourSpaces", async () => {
    const mockOnChange = vi.fn();
    const user = userEvent.setup();
    const { container } = render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);

    const select = container.querySelector("select") as HTMLSelectElement;
    await user.selectOptions(select, "FourSpaces");

    expect(mockOnChange).toHaveBeenCalledWith({
      indentation: "FourSpaces",
    });
  });

  it("calls onChange when selection changes to Tabs", async () => {
    const mockOnChange = vi.fn();
    const user = userEvent.setup();
    const { container } = render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);

    const select = container.querySelector("select") as HTMLSelectElement;
    await user.selectOptions(select, "Tabs");

    expect(mockOnChange).toHaveBeenCalledWith({
      indentation: "Tabs",
    });
  });

  it("calls onChange with correct config structure", async () => {
    const mockOnChange = vi.fn();
    const user = userEvent.setup();
    const { container } = render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);

    const select = container.querySelector("select") as HTMLSelectElement;
    await user.selectOptions(select, "FourSpaces");

    expect(mockOnChange).toHaveBeenCalledTimes(1);
    const callArg = mockOnChange.mock.calls[0][0];
    expect(callArg).toHaveProperty("indentation");
    expect(callArg.indentation).toBe("FourSpaces");
  });

  it("renders with default TwoSpaces selected initially", () => {
    const mockOnChange = vi.fn();
    const { container } = render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);
    const select = container.querySelector("select") as HTMLSelectElement;
    expect(select.value).toBe("TwoSpaces");
  });

  it("updates selected value when config prop changes", () => {
    const mockOnChange = vi.fn();
    const { rerender, container } = render(
      <ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />
    );

    let select = container.querySelector("select") as HTMLSelectElement;
    expect(select.value).toBe("TwoSpaces");

    const newConfig = { indentation: "Tabs" as const };
    rerender(<ConfigPanel config={newConfig} onChange={mockOnChange} />);

    select = container.querySelector("select") as HTMLSelectElement;
    expect(select.value).toBe("Tabs");
  });

  it("has proper styling classes", () => {
    const mockOnChange = vi.fn();
    const { container } = render(<ConfigPanel config={DEFAULT_CONFIG} onChange={mockOnChange} />);
    const select = container.querySelector("select");
    expect(select?.className).toContain("text-sm");
    expect(select?.className).toContain("border");
    expect(select?.className).toContain("rounded");
  });
});
