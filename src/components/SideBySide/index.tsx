import { useEffect, useState } from "react";
import Editor from "../Editor";
import DiffViewer from "../DiffViewer";
import { formatCode, initFormatter } from "~/utils/wasm";
import { randomPath, get } from "~/utils/nixpkgs";
import { getStateFromUrl, setStateInUrl } from "~/utils/permalink";

interface FormatterState {
  input: string;
  output: string;
}

export default function SideBySide() {
  const [state, setState] = useState<FormatterState>({
    input: "",
    output: "",
  });
  const [isLoading, setIsLoading] = useState(false);

  // Initialize formatter and load state
  useEffect(() => {
    const initialize = async () => {
      try {
        console.log("SideBySide: Initializing formatter...");
        await initFormatter();
        console.log("SideBySide: Formatter initialized");

        // Check if we have state in URL
        const urlState = getStateFromUrl();
        if (urlState?.code) {
          console.log("SideBySide: Loading from URL state");
          handleFormatCode(urlState.code);
        } else {
          // Load random Nix file
          console.log("SideBySide: Loading random file");
          loadRandomFile();
        }
      } catch (error) {
        console.error("SideBySide: Initialization failed:", error);
      }
    };

    initialize();
  }, []);

  const loadRandomFile = async () => {
    setIsLoading(true);
    try {
      const path = randomPath();
      const code = await get(path);
      handleFormatCode(code);
    } catch (error) {
      console.error("Failed to load random file:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleFormatCode = (code: string) => {
    setState({
      input: code,
      output: formatCode(code),
    });
    setStateInUrl({ code });
  };

  const handleInputChange = (newCode: string) => {
    handleFormatCode(newCode);
  };

  const handleLoadRandom = () => {
    loadRandomFile();
  };

  return (
    <div className="space-y-4">
      <div className="flex gap-4 flex-col sm:flex-row">
        <button
          onClick={handleLoadRandom}
          disabled={isLoading}
          className="rounded-lg bg-primary px-4 py-2 font-medium text-white hover:bg-blue-600 disabled:opacity-50"
        >
          {isLoading ? "Loading..." : "Load Random File"}
        </button>
      </div>

      <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <div className="space-y-2">
          <h2 className="text-lg font-semibold text-text-dark">Input</h2>
          <Editor
            value={state.input}
            onChange={handleInputChange}
            readOnly={false}
          />
        </div>

        <div className="space-y-2">
          <h2 className="text-lg font-semibold text-text-dark">Output</h2>
          <DiffViewer input={state.input} output={state.output} />
        </div>
      </div>
    </div>
  );
}
