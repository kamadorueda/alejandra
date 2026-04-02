import { useCallback, useEffect, useRef, useState } from "react";
import { formatCode, initFormatter } from "~/utils/wasm";
import { randomPath, get } from "~/utils/nixpkgs";
import { getStateFromUrl, setStateInUrl } from "~/utils/permalink";
import { FormatterConfig, DEFAULT_CONFIG } from "~/types/config";

interface FormatterState {
  input: string;
  output: string;
}

export function useFormatter() {
  const [state, setState] = useState<FormatterState>({
    input: "",
    output: "",
  });
  const [config, setConfig] = useState<FormatterConfig>(DEFAULT_CONFIG);
  const [isLoading, setIsLoading] = useState(false);
  const [wasmReady, setWasmReady] = useState(false);
  const [wasmError, setWasmError] = useState<string | null>(null);
  const [formattingError, setFormattingError] = useState<string | null>(null);
  const debounceTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Cleanup debounce timeout on unmount
  useEffect(() => {
    return () => {
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }
    };
  }, []);

  // Stable format code function (memoized) - uses current config
  const handleFormatCode = useCallback(
    (code: string) => {
      try {
        const formatted = formatCode(code, "file.nix", config);
        setState({
          input: code,
          output: formatted,
        });
        setFormattingError(null);
        setStateInUrl({ code, config });
      } catch (error) {
        const errorMsg = error instanceof Error ? error.message : String(error);
        console.error("useFormatter: Formatting error:", error);
        setFormattingError(errorMsg);
        setState((prev) => ({
          ...prev,
          input: code,
        }));
      }
    },
    [config]
  );

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
          // Load config from URL (always present due to backward compatibility in decodeState)
          setConfig(urlState.config);
          // Format with loaded config directly (can't use handleFormatCode due to async state update)
          try {
            const formatted = formatCode(urlState.code, "file.nix", urlState.config);
            setState({
              input: urlState.code,
              output: formatted,
            });
            setFormattingError(null);
          } catch (error) {
            const errorMsg = error instanceof Error ? error.message : String(error);
            console.error("useFormatter: Formatting error on URL state:", error);
            setFormattingError(errorMsg);
            setState({
              input: urlState.code,
              output: "",
            });
          }
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
        try {
          const formatted = formatCode(newCode, "file.nix", config);
          setState((prev) => ({
            ...prev,
            output: formatted,
          }));
          setFormattingError(null);
        } catch (error) {
          const errorMsg = error instanceof Error ? error.message : String(error);
          console.error("useFormatter: Formatting error on input change:", error);
          setFormattingError(errorMsg);
          setState((prev) => ({
            ...prev,
            output: "",
          }));
        }
        setStateInUrl({ code: newCode, config });
      }, 300);
    },
    [config]
  );

  const handleConfigChange = useCallback(
    (newConfig: FormatterConfig) => {
      setConfig(newConfig);
      // Re-format with the new config
      try {
        const formatted = formatCode(state.input, "file.nix", newConfig);
        setState((prev) => ({
          ...prev,
          output: formatted,
        }));
        setFormattingError(null);
      } catch (error) {
        const errorMsg = error instanceof Error ? error.message : String(error);
        console.error("useFormatter: Formatting error on config change:", error);
        setFormattingError(errorMsg);
        setState((prev) => ({
          ...prev,
          output: "",
        }));
      }
      setStateInUrl({ code: state.input, config: newConfig });
    },
    [state.input]
  );

  return {
    state,
    config,
    isLoading,
    wasmReady,
    wasmError,
    formattingError,
    handleFormatCode,
    handleInputChange,
    handleConfigChange,
    loadRandomFile,
  };
}
