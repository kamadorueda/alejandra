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
  return btoa(json);
};

/**
 * Decode state from a URL-safe string
 */
export const decodeState = (encoded: string): PermalinkState | null => {
  try {
    const json = atob(encoded);
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
