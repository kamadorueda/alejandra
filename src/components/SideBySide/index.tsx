import { useEffect, useState, useRef } from "react";
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
  const [wasmReady, setWasmReady] = useState(false);
  const [wasmError, setWasmError] = useState<string | null>(null);
  const debounceTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Cleanup debounce timeout on unmount
  useEffect(() => {
    return () => {
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }
    };
  }, []);

  // Initialize formatter and load state
  useEffect(() => {
    const initialize = async () => {
      try {
        await initFormatter();
        setWasmReady(true);
        setWasmError(null);
      } catch (error) {
        const errorMsg = error instanceof Error ? error.message : String(error);
        console.error("SideBySide: Formatter initialization failed:", error);
        setWasmError(errorMsg);
        return;
      }

      try {
        // Check if we have state in URL
        const urlState = getStateFromUrl();
        if (urlState?.code) {
          handleFormatCode(urlState.code);
        } else {
          // Load random Nix file
          await loadRandomFile();
        }
      } catch (error) {
        console.error("SideBySide: File loading failed:", error);
        // Fallback: load default code
        const defaultCode = '{ lib, stdenv }:\n\nstdenv.mkDerivation {\n  name = "example";\n  src = ./.;\n}';
        handleFormatCode(defaultCode);
      }
    };

    initialize();
  }, []);

  const loadRandomFile = async () => {
    setIsLoading(true);
    try {
      const path = await randomPath();
      const code = await get(path);
      handleFormatCode(code);
    } catch (error) {
      console.error("Failed to load random file:", error);
      // Fallback: use default example
      const defaultCode = '{ lib, stdenv }:\n\nstdenv.mkDerivation {\n  name = "example";\n  src = ./.;\n}';
      handleFormatCode(defaultCode);
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
    // Update input immediately for responsive typing
    setState((prev) => ({
      ...prev,
      input: newCode,
    }));

    // Debounce formatting and diff update by 300ms
    if (debounceTimeoutRef.current) {
      clearTimeout(debounceTimeoutRef.current);
    }

    debounceTimeoutRef.current = setTimeout(() => {
      setState((prev) => ({
        ...prev,
        output: formatCode(newCode),
      }));
      setStateInUrl({ code: newCode });
    }, 300);
  };

  const handleLoadRandom = () => {
    loadRandomFile();
  };

  // Show loading state while WASM is initializing
  if (!wasmReady && !wasmError) {
    return (
      <div className="flex flex-col items-center justify-center py-16">
        <div className="text-center">
          <div className="inline-block animate-spin mb-4">
            <div className="h-8 w-8 border-4 border-primary border-t-transparent rounded-full"></div>
          </div>
          <p className="text-sm text-text-dark">Initializing formatter...</p>
        </div>
      </div>
    );
  }

  // Show error state if WASM initialization failed
  if (wasmError) {
    const handleRetry = () => {
      setWasmError(null);
      setWasmReady(false);
      window.location.reload();
    };

    return (
      <div className="flex flex-col items-center justify-center py-16">
        <div className="text-center max-w-md">
          <div className="mb-4 text-red-600 text-lg">⚠️</div>
          <h2 className="text-lg font-semibold text-text-dark mb-2">Formatter Failed to Initialize</h2>
          <p className="text-sm text-text mb-4">{wasmError}</p>
          <button
            onClick={handleRetry}
            className="px-4 py-2 bg-primary text-white text-sm rounded hover:opacity-90"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

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
