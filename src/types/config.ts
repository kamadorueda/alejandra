export type Indentation = "TwoSpaces" | "FourSpaces" | "Tabs";

export interface FormatterConfig {
  indentation: Indentation;
}

export const DEFAULT_CONFIG: FormatterConfig = {
  indentation: "TwoSpaces",
};
