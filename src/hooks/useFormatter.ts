import { useCallback, useEffect, useRef, useState } from "react";
import { formatCode, initFormatter } from "~/utils/wasm";
import { randomPath, get } from "~/utils/nixpkgs";
import { getStateFromUrl, setStateInUrl } from "~/utils/permalink";

interface FormatterState {
  input: string;
  output: string;
}

export function useFormatter() {
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

  // Stable format code function (memoized)
  const handleFormatCode = useCallback((code: string) => {
    setState({
      input: code,
      output: formatCode(code),
    });
    setStateInUrl({ code });
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
        console.error("useFormatter: Formatter initialization failed:", error);
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
        console.error("useFormatter: File loading failed:", error);
        // Fallback: load default code
        const defaultCode = '{ lib, stdenv }:\n\nstdenv.mkDerivation {\n  name = "example";\n  src = ./.;\n}';
        handleFormatCode(defaultCode);
      }
    };

    initialize();
  }, [handleFormatCode]);

  const loadRandomFile = async () => {
    setIsLoading(true);
    try {
      const path = await randomPath();
      const code = await get(path);
      handleFormatCode(code);
    } catch (error) {
      console.error("useFormatter: Failed to load random file:", error);
      // Fallback: use default example
      const defaultCode = '{ lib, stdenv }:\n\nstdenv.mkDerivation {\n  name = "example";\n  src = ./.;\n}';
      handleFormatCode(defaultCode);
    } finally {
      setIsLoading(false);
    }
  };

  const handleInputChange = useCallback(
    (newCode: string) => {
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
    },
    []
  );

  return {
    state,
    isLoading,
    wasmReady,
    wasmError,
    handleFormatCode,
    handleInputChange,
    loadRandomFile,
  };
}
