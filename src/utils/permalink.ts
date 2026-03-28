/**
 * Permalink utilities for sharing formatter state via URL
 */

interface PermalinkState {
  code: string;
}

/**
 * Encode state into a URL-safe string
 */
export const encodeState = (state: PermalinkState): string => {
  const json = JSON.stringify(state);
  // Handle unicode characters by converting to UTF-8 bytes first
  const uint8Array = new TextEncoder().encode(json);
  let binary = '';
  for (let i = 0; i < uint8Array.length; i++) {
    binary += String.fromCharCode(uint8Array[i]);
  }
  return btoa(binary);
};

/**
 * Decode state from a URL-safe string
 */
export const decodeState = (encoded: string): PermalinkState | null => {
  try {
    const binary = atob(encoded);
    // Convert binary string back to UTF-8 bytes
    const uint8Array = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) {
      uint8Array[i] = binary.charCodeAt(i);
    }
    const json = new TextDecoder().decode(uint8Array);
    return JSON.parse(json) as PermalinkState;
  } catch {
    return null;
  }
};

/**
 * Get state from URL hash
 */
export const getStateFromUrl = (): PermalinkState | null => {
  const hash = window.location.hash.slice(1); // Remove leading #
  if (!hash) return null;
  return decodeState(hash);
};

/**
 * Set state in URL hash
 */
export const setStateInUrl = (state: PermalinkState): void => {
  history.replaceState(null, "", "#" + encodeState(state));
};
