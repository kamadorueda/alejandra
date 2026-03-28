import React, { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export default class ErrorBoundary extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error("Error caught by boundary:", error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="min-h-screen bg-red-50 py-8">
          <div className="mx-auto max-w-4xl px-4">
            <h1 className="text-2xl font-bold text-red-900">Something went wrong</h1>
            <p className="mt-2 text-red-700">{this.state.error?.message}</p>
            <details className="mt-4 text-sm text-red-600">
              <summary>Stack trace</summary>
              <pre className="mt-2 overflow-auto rounded bg-red-100 p-2">
                {this.state.error?.stack}
              </pre>
            </details>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
