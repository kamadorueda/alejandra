import Editor from "../Editor";
import DiffViewer from "../DiffViewer";
import { useFormatter } from "~/hooks/useFormatter";

export default function SideBySide() {
  const { state, isLoading, wasmReady, wasmError, handleInputChange, loadRandomFile } = useFormatter();

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
            onClick={loadRandomFile}
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
