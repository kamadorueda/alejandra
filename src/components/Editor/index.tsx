import { useRef, useEffect } from "react";
import * as CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";
import "codemirror/theme/monokai.css";
import "codemirror/mode/shell/shell";

interface EditorProps {
  value: string;
  onChange: (value: string) => void;
  readOnly?: boolean;
}

export default function Editor({ value, onChange, readOnly = false }: EditorProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const editorRef = useRef<CodeMirror.Editor | null>(null);

  // Initialize editor once
  useEffect(() => {
    if (!containerRef.current || editorRef.current) return;

    // Create textarea element and add to DOM first
    const textarea = document.createElement("textarea");
    containerRef.current.appendChild(textarea);

    // Create editor
    const editor = CodeMirror.fromTextArea(textarea, {
      mode: "shell",
      theme: "monokai",
      lineNumbers: true,
      indentUnit: 2,
      readOnly: readOnly,
      viewportMargin: Infinity,
    });

    editorRef.current = editor;

    return () => {
      editor.toTextArea();
      editorRef.current = null;
    };
  }, [readOnly]);

  // Update onChange handler
  useEffect(() => {
    if (!editorRef.current) return;

    const changeHandler = () => {
      onChange(editorRef.current!.getValue());
    };

    editorRef.current.on("change", changeHandler);

    return () => {
      editorRef.current?.off("change", changeHandler);
    };
  }, [onChange]);

  // Update content when value prop changes (external updates)
  useEffect(() => {
    if (editorRef.current && editorRef.current.getValue() !== value) {
      editorRef.current.setValue(value);
    }
  }, [value]);

  return <div ref={containerRef} className="bg-editor-bg rounded-lg overflow-hidden min-h-96" style={{ backgroundColor: '#2d2d2d' }} />;
}
