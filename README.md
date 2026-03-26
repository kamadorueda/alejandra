# Alejandra Frontend - Modern Stack

A modernized frontend for Alejandra Nix formatter using:
- **Vite** for fast development and optimized builds
- **TypeScript** for type safety
- **React 19** with modern patterns
- **Tailwind CSS v4** for styling
- **pnpm** for package management
- **Nix flakes** for reproducible environments
- **CodeMirror 5** for code editing
- **WASM** integration for the formatter

## Features

- Side-by-side code editor and diff viewer
- Format Nix code with Alejandra formatter
- Load random Nix files from nixpkgs for testing
- Permalink sharing via URL hash
- Modern, responsive UI

## Development

### Setup

```bash
# With direnv (recommended)
direnv allow

# Or manually enter the dev environment
nix flake show
nix develop
```

### Commands

```bash
# Install dependencies
pnpm install

# Start development server
pnpm dev

# Build for production
pnpm build

# Preview production build
pnpm preview
```

## Project Structure

```
src/
├── components/
│   ├── Editor/         # CodeMirror editor component
│   ├── SideBySide/     # Main layout container
│   └── DiffViewer/     # Diff display component
├── utils/
│   ├── wasm.ts         # WASM formatter wrapper
│   ├── nixpkgs.ts      # Nixpkgs file fetching utilities
│   ├── nixpkgsFiles.ts # List of nixpkgs files
│   └── permalink.ts    # URL state management
├── types/
│   └── wasm.d.ts       # WASM module type definitions
├── App.tsx             # Main application component
├── index.tsx           # React entry point
└── index.css           # Tailwind styles with custom theme
```

## Configuration Files

- `vite.config.ts` - Vite build configuration
- `tailwind.config.ts` - Tailwind CSS configuration (minimal for v4)
- `tsconfig.json` - TypeScript configuration
- `flake.nix` - Nix development environment
- `.envrc` - direnv configuration
- `index.html` - HTML entry point
