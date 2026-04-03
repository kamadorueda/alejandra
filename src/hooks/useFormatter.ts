import { useCallback, useEffect, useRef, useState } from "react";
import { formatCode, initFormatter } from "~/utils/wasm";
import { getRandomFile } from "~/utils/nixpkgs";
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
  const urlUpdateTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Cleanup debounce timeouts on unmount
  useEffect(() => {
    return () => {
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }
      if (urlUpdateTimeoutRef.current) {
        clearTimeout(urlUpdateTimeoutRef.current);
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
        // Defer URL update to avoid blocking formatting
        setTimeout(() => setStateInUrl({ code, config }), 0);
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
      const { content: code } = await getRandomFile();
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

      // Debounce formatting by 300ms
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
      }, 300);

      // Debounce URL update by 1s (separate from formatting to avoid blocking)
      if (urlUpdateTimeoutRef.current) {
        clearTimeout(urlUpdateTimeoutRef.current);
      }

      urlUpdateTimeoutRef.current = setTimeout(() => {
        setStateInUrl({ code: newCode, config });
      }, 1000);
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
      // Defer URL update to avoid blocking formatting
      setTimeout(() => setStateInUrl({ code: state.input, config: newConfig }), 0);
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
