import { useRef, useEffect } from "react";
import * as CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";

interface EditorProps {
  value: string;
  onChange: (value: string) => void;
  readOnly?: boolean;
}

export default function Editor({ value, onChange, readOnly = false }: EditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const editorRef = useRef<CodeMirror.Editor | null>(null);

  useEffect(() => {
    if (!containerRef.current) return;

    // Create editor
    const editor = CodeMirror.fromTextArea(
      document.createElement("textarea"),
      {
        mode: "nix",
        theme: "default",
        lineNumbers: true,
        indentUnit: 2,
        readOnly: readOnly,
        viewportMargin: Infinity,
      }
    );

    containerRef.current.appendChild(editor.getWrapperElement());
    editorRef.current = editor;

    // Handle changes
    const changeHandler = () => {
      onChange(editor.getValue());
    };

    editor.on("change", changeHandler);

    return () => {
      editor.off("change", changeHandler);
      editor.getWrapperElement().remove();
    };
  }, [onChange, readOnly]);

  // Update content when value prop changes (external updates)
  useEffect(() => {
    if (editorRef.current && editorRef.current.getValue() !== value) {
      editorRef.current.setValue(value);
    }
  }, [value]);

  return <div ref={containerRef} className="border border-neutral-200 rounded-lg overflow-hidden" />;
}
