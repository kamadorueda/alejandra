import { FormatterConfig, Indentation } from "~/types/config";

interface ConfigPanelProps {
  config: FormatterConfig;
  onChange: (config: FormatterConfig) => void;
}

export default function ConfigPanel({ config, onChange }: ConfigPanelProps) {
  const handleIndentationChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newIndentation = e.target.value as Indentation;
    onChange({ ...config, indentation: newIndentation });
  };

  return (
    <div className="flex items-center gap-2">
      <label htmlFor="indentation-select" className="text-sm font-medium text-text-dark">
        Indentation:
      </label>
      <select
        id="indentation-select"
        value={config.indentation}
        onChange={handleIndentationChange}
        className="text-sm border border-neutral-200 rounded px-2 py-1 bg-white text-text hover:border-neutral-300 focus:outline-none focus:ring-2 focus:ring-primary focus:ring-opacity-50"
      >
        <option value="TwoSpaces">2 Spaces</option>
        <option value="FourSpaces">4 Spaces</option>
        <option value="Tabs">Tabs</option>
      </select>
    </div>
  );
}
