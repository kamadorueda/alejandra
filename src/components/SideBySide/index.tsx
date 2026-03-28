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
    <div className="space-y-6">
      {/* Instructions section */}
      <div className="border-b border-neutral-200 pb-4">
        <div className="text-sm text-text">
          Type your code below or{" "}
          <button
            onClick={handleLoadRandom}
            disabled={isLoading}
            className="text-primary hover:underline font-medium"
          >
            {isLoading ? "loading..." : "click here to fetch a random file from Nixpkgs"}
          </button>
        </div>
      </div>

      {/* Input and Output editors side-by-side */}
      <div className="grid grid-cols-1 gap-4 lg:grid-cols-2">
        <div className="space-y-2">
          <h2 className="text-sm font-semibold text-text-dark uppercase tracking-wide">Input</h2>
          <Editor
            value={state.input}
            onChange={handleInputChange}
            readOnly={false}
          />
        </div>

        <div className="space-y-2">
          <div className="text-sm font-medium text-text-dark">
            With Alejandra ❤️
          </div>
          <Editor
            value={state.output}
            onChange={() => {}}
            readOnly={true}
          />
        </div>
      </div>

      {/* Git patch diff viewer */}
      <DiffViewer input={state.input} output={state.output} />

      {/* Permalink section */}
      <div className="flex items-center gap-2 text-sm text-text border-t border-neutral-200 pt-4">
        <span>Permalink:</span>
        <a
          href={`#${window.location.hash.slice(1)}`}
          className="text-primary hover:underline font-medium"
        >
          here
        </a>
      </div>
    </div>
  );
}
