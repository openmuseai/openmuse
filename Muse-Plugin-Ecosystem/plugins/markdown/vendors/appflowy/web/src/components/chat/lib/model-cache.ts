/**
 * Model selection cache management
 * Stores selected models per chat in localStorage for quick access
 */

const CACHE_KEY_PREFIX = 'af_chat_model_';
const CACHE_EXPIRY_MS = 24 * 60 * 60 * 1000; // 24 hours

interface CachedModel {
  modelName: string;
  timestamp: number;
}

export class ModelCache {
  /**
   * Get cached model for a specific chat
   */
  static get(chatId: string): string | null {
    try {
      const key = `${CACHE_KEY_PREFIX}${chatId}`;
      const cached = localStorage.getItem(key);
      
      if (!cached) return null;
      
      const data: CachedModel = JSON.parse(cached);
      
      // Check if cache is expired
      if (Date.now() - data.timestamp > CACHE_EXPIRY_MS) {
        localStorage.removeItem(key);
        return null;
      }
      
      return data.modelName;
    } catch (error) {
      console.error('Error reading model cache:', error);
      return null;
    }
  }

  /**
   * Set cached model for a specific chat
   */
  static set(chatId: string, modelName: string): void {
    try {
      const key = `${CACHE_KEY_PREFIX}${chatId}`;
      const data: CachedModel = {
        modelName,
        timestamp: Date.now(),
      };

      localStorage.setItem(key, JSON.stringify(data));
    } catch (error) {
      console.error('Error setting model cache:', error);
    }
  }

  /**
   * Clear cached model for a specific chat
   */
  static clear(chatId: string): void {
    try {
      const key = `${CACHE_KEY_PREFIX}${chatId}`;

      localStorage.removeItem(key);
    } catch (error) {
      console.error('Error clearing model cache:', error);
    }
  }

  /**
   * Clear all cached models
   */
  static clearAll(): void {
    try {
      const keys = Object.keys(localStorage);

      keys.forEach(key => {
        if (key.startsWith(CACHE_KEY_PREFIX)) {
          localStorage.removeItem(key);
        }
      });
    } catch (error) {
      console.error('Error clearing all model cache:', error);
    }
  }
}