declare module "alejandra_front" {
  export default function init(): Promise<void>;
  export function format(code: string, filename: string): string;
}
