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

    // Create textarea element and add to DOM first
    const textarea = document.createElement("textarea");
    containerRef.current.appendChild(textarea);

    // Create editor
    const editor = CodeMirror.fromTextArea(textarea, {
      mode: "nix",
      theme: "default",
      lineNumbers: true,
      indentUnit: 2,
      readOnly: readOnly,
      viewportMargin: Infinity,
    });

    editorRef.current = editor;

    // Handle changes
    const changeHandler = () => {
      onChange(editor.getValue());
    };

    editor.on("change", changeHandler);

    return () => {
      editor.off("change", changeHandler);
      editor.toTextArea();
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
